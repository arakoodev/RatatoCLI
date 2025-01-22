import { AzureFunction, Context, HttpRequest } from "@azure/functions";
import { TableClient, AzureNamedKeyCredential } from "@azure/data-tables";
import { DefaultAzureCredential } from "@azure/identity";
import { SecretClient } from "@azure/keyvault-secrets";
import axios from "axios";

interface StoreValidationResponse {
    isValid: boolean;
    tier: string;
    quotaLimit: number;
}

interface UsageRecord {
    partitionKey: string;  // userId
    rowKey: string;        // yearMonth
    count: number;
    tier: string;
}

interface AnthropicMessage {
    role: string;
    content: string;
}

interface AnthropicRequest {
    model: string;
    messages: AnthropicMessage[];
    max_tokens: number;
}

const httpTrigger: AzureFunction = async function (context: Context, req: HttpRequest): Promise<void> {
    try {
        // 1. Extract and validate headers
        const storeToken = req.headers["x-store-token"];
        const userId = req.headers["x-user-id"];

        if (!storeToken || !userId) {
            context.res = {
                status: 401,
                body: { error: "Missing authentication headers" }
            };
            return;
        }

        // 2. Validate Windows Store license
        const storeValidation = await validateStoreLicense(storeToken);
        if (!storeValidation.isValid) {
            context.res = {
                status: 401,
                body: { error: "Invalid or expired license" }
            };
            return;
        }

        // 3. Check usage quota
        const usage = await checkAndUpdateUsage(userId, storeValidation);
        if (!usage.allowed) {
            context.res = {
                status: 429,
                body: { error: "Monthly quota exceeded" }
            };
            return;
        }

        // 4. Get Anthropic key from Key Vault
        const anthropicKey = await getAnthropicKey();

        // 5. Forward request to Anthropic
        const response = await callAnthropic(req.body, anthropicKey);

        // 6. Return response
        context.res = {
            status: 200,
            body: response
        };

    } catch (error) {
        context.log.error("Error processing request:", error);
        context.res = {
            status: 500,
            body: { error: "Internal server error" }
        };
    }
};

async function validateStoreLicense(token: string): Promise<StoreValidationResponse> {
    // In production, validate against Windows Store API
    // For now, returning mock data based on token
    const tiers = {
        "free": { limit: 50 },
        "basic": { limit: 500 },
        "pro": { limit: 2000 },
        "enterprise": { limit: 10000 }
    };

    // Mock validation - in production, actually validate against Windows Store
    return {
        isValid: true,
        tier: "basic",
        quotaLimit: tiers["basic"].limit
    };
}

async function checkAndUpdateUsage(userId: string, validation: StoreValidationResponse): Promise<{ allowed: boolean }> {
    const tableClient = TableClient.fromConnectionString(
        process.env.AzureWebJobsStorage || "",
        "usageQuota"
    );

    const yearMonth = new Date().toISOString().slice(0, 7);
    
    try {
        const record = await tableClient.getEntity<UsageRecord>(userId, yearMonth);
        
        if (record.count >= validation.quotaLimit) {
            return { allowed: false };
        }

        // Update usage
        await tableClient.updateEntity({
            partitionKey: userId,
            rowKey: yearMonth,
            count: record.count + 1,
            tier: validation.tier
        }, "Merge");

        return { allowed: true };

    } catch (error: any) {
        if (error.statusCode === 404) {
            // Create new record
            await tableClient.createEntity({
                partitionKey: userId,
                rowKey: yearMonth,
                count: 1,
                tier: validation.tier
            });
            return { allowed: true };
        }
        throw error;
    }
}

async function getAnthropicKey(): Promise<string> {
    const credential = new DefaultAzureCredential();
    const keyVaultName = process.env.KEY_VAULT_NAME;
    const keyVaultUrl = `https://${keyVaultName}.vault.azure.net`;
    
    const keyVaultClient = new SecretClient(keyVaultUrl, credential);
    const secret = await keyVaultClient.getSecret("AnthropicKey");
    
    return secret.value || "";
}

async function callAnthropic(request: AnthropicRequest, apiKey: string) {
    const response = await axios.post(
        "https://api.anthropic.com/v1/messages",
        request,
        {
            headers: {
                "x-api-key": apiKey,
                "anthropic-version": "2023-06-01"
            }
        }
    );

    return response.data;
}

export default httpTrigger;

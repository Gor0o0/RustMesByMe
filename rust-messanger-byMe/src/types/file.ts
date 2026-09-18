import { convertFileSrc } from "@tauri-apps/api/core";
import { resolve, isAbsolute } from "@tauri-apps/api/path";

export async function getFileUrl(filePath: string): Promise<string> {
    if (!filePath) return "";

    if (
        filePath.startsWith("http://") ||
        filePath.startsWith("https://") ||
        filePath.startsWith("data:") ||
        filePath.startsWith("asset://") ||
        filePath.startsWith("file://")
    ) {
        return filePath;
    }

    try {
        const absolute = await (async () => {
            if (await isAbsolute(filePath)) {
                return filePath;
            }
            return resolve(".", filePath);
        })();

        return convertFileSrc(absolute);
    } catch (_e) {
        try {
            return convertFileSrc(filePath);
        } catch (_e2) {
            return filePath;
        }
    }
}

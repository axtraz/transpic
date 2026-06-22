import { save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";

export async function downloadImage(
  processedUrl: string,
  imageName: string | null,
  format: string
): Promise<void> {
  const base64 = processedUrl.split(",")[1];
  const bytes = Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));

  const path = await save({
    defaultPath: `${imageName?.split(".")[0] ?? "transpic-output"}.${format}`,
    filters: [{ name: "Image", extensions: [format] }],
  });

  if (path) {
    await writeFile(path, bytes);
  }
}
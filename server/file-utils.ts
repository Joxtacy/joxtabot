export enum PATHS {
    FIRST = "./obs/first.txt",
}

export const writeToFile = async (path: PATHS, text: string): Promise<void> => {
    const encoder = new TextEncoder();
    const encodedText = encoder.encode(text);

    await Deno.writeFile(path, encodedText);
};

import { PATHS, writeToFile } from "./file-utils.ts";

export const writeFirst = async (userName: string) => {
  await writeToFile(PATHS.FIRST, `First: ${userName}`);
};

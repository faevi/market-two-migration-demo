import { pathsToModuleNameMapper } from "ts-jest";
import { compilerOptions } from "./tsconfig.json";

export default {
  preset: "ts-jest",
  testEnvironment: "node",
  testTimeout: 30000,
  moduleDirectories: ["node_modules", "./src"],
  moduleFileExtensions: ["js", "ts", "json"],
  moduleNameMapper: pathsToModuleNameMapper(compilerOptions.paths, {
    prefix: "<rootDir>/",
  }),
  transform: {
    "^.+\\.(ts|tsx)?$": [
      "ts-jest",
      { tsconfig: "tsconfig.json" },
    ],
  },
};
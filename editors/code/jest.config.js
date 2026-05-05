/* SPDX-License-Identifier: BSD-3-Clause */
/** @type {import('ts-jest').JestConfigWithTsJest} */
module.exports =
{
  preset: 'ts-jest',
  testEnvironment: 'node',
  rootDir: 'test',
  testRegex: ['.*\\.ts']
};

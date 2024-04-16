export const handleGenericError = (functionName: string, error: Error) => {
  throw new Error(`[${functionName}] -  ${error}`)
}

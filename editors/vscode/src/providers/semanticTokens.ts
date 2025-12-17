import {
	CancellationError,
	CancellationToken,
	DocumentSemanticTokensProvider,
	SemanticTokens,
	SemanticTokensBuilder,
	SemanticTokensLegend,
	TextDocument,
} from 'vscode'
import {ClientWorkspace} from '../extension'
import {
	RequestType,
	SemanticTokenModifiers,
	SemanticTokensParams,
	SemanticTokensRequest,
} from 'vscode-languageclient'
import {GetSemanticTokensResult, SemanticTokenTypes} from '../types/semanticTokens'

const getSemanticTokensRequest =
	new RequestType<SemanticTokensParams, GetSemanticTokensResult, void>(SemanticTokensRequest.method)

export class SemanticTokensProvider implements DocumentSemanticTokensProvider
{
	private workspace: ClientWorkspace
	private tokensLegend: SemanticTokensLegend

	constructor(workspace: ClientWorkspace)
	{
		this.workspace = workspace

		const tokenTypes: string[] = []
		for (const value in SemanticTokenTypes)
		{
			if (isNaN(Number(value)))
				tokenTypes.push(value)
		}

		const tokenModifiers: string[] = []
		for (const value in SemanticTokenModifiers)
			tokenModifiers.push(value)

		this.tokensLegend = new SemanticTokensLegend(tokenTypes, tokenModifiers)
	}

	get legend()
	{
		return this.tokensLegend
	}

	async provideDocumentSemanticTokens(document: TextDocument, token: CancellationToken):
		Promise<SemanticTokens>
	{
		await this.workspace.awaitReady()
		const params: SemanticTokensParams = {textDocument: {uri: document.uri.toString()}}
		const result: GetSemanticTokensResult =
			await this.workspace.languageClient.sendRequest(getSemanticTokensRequest, params, token)

		if (result.canceled)
			throw new CancellationError()
		const builder: SemanticTokensBuilder = new SemanticTokensBuilder(this.tokensLegend)

		for (const semanticToken of result.tokens)
		{
			builder.push(
				semanticToken.line,
				semanticToken.character,
				semanticToken.length,
				semanticToken.type,
				semanticToken.modifiers,
			)
		}
		return builder.build()
	}
}

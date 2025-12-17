import {commands, Disposable, ExtensionContext, languages, WorkspaceFolder} from 'vscode'
import * as langClient from 'vscode-languageclient'
import {LanguageClient} from 'vscode-languageclient/node'
import {createLanguageClient, documentSelector, setupProgress} from './vermilion'
import {Observable} from './utils/observable'
// import {startSpinner, stopSpinner} from './utils/spinner'
import {SemanticTokensProvider} from './providers/semanticTokens'

const workspaces: Map<string, ClientWorkspace> = new Map()
const activeWorkspace = new Observable<ClientWorkspace | null>(null)
export let extensionContext: ExtensionContext
// let progress: Disposable

export type WorkspaceProgress = {state: 'progress'; message: string} | {state: 'ready' | 'standby'}

/* exported activate */
export async function activate(context: ExtensionContext)
{
	extensionContext = context
	context.subscriptions.push(
		...[
			configureLanguage(),
			...registerCommands(),
		],
	)

	return {activeWorkspace}
}

/* exported deactivate */
export async function deactivate()
{
	return Promise.all([...workspaces.values()].map(workspace => workspace.stop()))
}

export class ClientWorkspace
{
	public readonly folder: WorkspaceFolder
	private client: LanguageClient | null = null
	private clientStarted: Thenable<void>
	private disposables: Disposable[]
	private _progress: Observable<WorkspaceProgress>

	constructor(folder: WorkspaceFolder)
	{
		this.folder = folder
		this.disposables = []
		this._progress = new Observable<WorkspaceProgress>({state: 'standby'})
		this.clientStarted = this.start()
	}

	get progress()
	{
		return this._progress
	}

	get languageClient()
	{
		if (!this.client)
			throw new Error('Attempting to use languageClient before it is initialised')
		return this.client
	}

	public async start()
	{
		const client = await createLanguageClient(this.folder)
		client.onDidChangeState(({newState}) =>
		{
			if (newState === langClient.State.Starting)
				this._progress.value = {state: 'progress', message: 'Starting'}
			else if (newState === langClient.State.Running)
				this._progress.value = {state: 'ready'}
			else if (newState === langClient.State.Stopped)
				this._progress.value = {state: 'standby'}
		})

		this.client = client
		setupProgress(client, this._progress)
		//this.disposables.push(activateTaskProvder(this.folder))
		this.disposables.push(...this.setupClient())
	}

	public async stop()
	{
		if (this.client)
			await this.client.stop()
		this.disposables.forEach(item => void item.dispose())
	}

	public async restart()
	{
		await this.stop()
		return this.start()
	}

	public awaitReady()
	{
		return this.clientStarted
	}

	setupClient(): Disposable[]
	{
		const semanticTokensProvider = new SemanticTokensProvider(this)

		return [
			languages.registerDocumentSemanticTokensProvider(
				documentSelector,
				semanticTokensProvider,
				semanticTokensProvider.legend,
			),
		]
	}
}

/* exported registerCommands */
function registerCommands(): Disposable[]
{
	return [
		commands.registerCommand(
			'vermilion.restart',
			async () => activeWorkspace.value?.restart(),
		),
	]
}

/* exported configureLanguage */
function configureLanguage(): Disposable
{
	return languages.setLanguageConfiguration('verilog', {})
}

import * as path from "path";
import * as vscode from "vscode";
import * as which from "which";
import * as util from "util";

import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient";

// this method is called when your extension is activated
// your extension is activated the very first time the command is executed
export async function activate(context: vscode.ExtensionContext) {
  console.log('starting catlang server...');
  await tryActivate(context).catch((err) => {
    vscode.window.showErrorMessage(
      `Cannot activate vscode-catlang: ${err.message}`
    );
    throw err;
  });
}

async function tryActivate(context: vscode.ExtensionContext) {
  // Use path variable
  const whichPromise = util.promisify(which);
  const catlangPath = (await whichPromise("catlang")) as Array<string>;
  console.log(`found catlang at ${catlangPath[0]}`);

  const executable = catlangPath;
  const args: string[] = ["start-language-server"];

  let serverOptions: ServerOptions = {
    command: executable[0],
    args,
    options: {},
  };

  let clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "catlang" }],
  };

  let disposable = new LanguageClient(
    "catlangLS",
    "Catlang Language Server",
    serverOptions,
    clientOptions
  ).start();

  context.subscriptions.push(disposable);
}

// this method is called when your extension is deactivated
export function deactivate() {
  console.log("catlang is now deactivated!");
}

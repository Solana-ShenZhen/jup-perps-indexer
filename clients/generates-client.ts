import {
  AnchorIdl,
  rootNodeFromAnchorWithoutDefaultVisitor,
} from "@kinobi-so/nodes-from-anchor";
import { renderRustVisitor } from "@kinobi-so/renderers";
import { visit } from "@kinobi-so/visitors-core";
import * as fs from "fs";
import * as path from "path";

const anchorIdl = JSON.parse(
  fs.readFileSync(path.join(__dirname, "../idl/perpetuals.json"), "utf-8")
);

async function generateClients() {
  const anchorIdl = JSON.parse(
    fs.readFileSync(path.join(__dirname, "../idl/perpetuals.json"), "utf-8")
  ) as AnchorIdl;

  const node = rootNodeFromAnchorWithoutDefaultVisitor(anchorIdl);

  const clients = [
    {
      type: "Rust",
      dir: "clients/generated/rust/src",
      renderVisitor: renderRustVisitor,
    },
  ];

  for (const client of clients) {
    try {
      await visit(node, await client.renderVisitor(client.dir));
      console.log(
        `✅ Successfully generated ${client.type} client for directory: ${client.dir}!`
      );
    } catch (e) {
      console.error(`Error in ${client.renderVisitor.name}:`, e);
      throw e;
    }
  }
}

generateClients();

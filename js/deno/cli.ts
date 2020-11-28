import { is_bree } from "./is_bree.ts";

const args = Deno.args;

if (args.length < 3) {
  console.log("Usage: is_bree <yyyy> <mm> <dd>");
  Deno.exit(0);
}

const date = args
  .slice(0, 3)
  .map((a) => a.padStart(2, "0"))
  .join("-");

try {
  console.log(`is_bree(${date}): ${is_bree(date)}`);
} catch(err) {
  console.error(err.message)
  Deno.exit(0);
}
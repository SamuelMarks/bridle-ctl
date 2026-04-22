const fs = require("fs");
const cheerio = require("cheerio");
const html = fs.readFileSync("coverage/bridle-ui/index.html");
const $ = cheerio.load(html);
let failed = false;
$("table.coverage-summary tbody tr").each((i, el) => {
  const file = $(el).find("td").eq(0).text().trim();
  const stmts = $(el).find("td").eq(2).text().trim();
  const branches = $(el).find("td").eq(4).text().trim();
  const funcs = $(el).find("td").eq(6).text().trim();
  const lines = $(el).find("td").eq(8).text().trim();
  if (
    lines !== "100%" ||
    branches !== "100%" ||
    funcs !== "100%" ||
    stmts !== "100%"
  ) {
    console.log(
      `${file} | Stmts: ${stmts} | Branches: ${branches} | Funcs: ${funcs} | Lines: ${lines}`,
    );
    failed = true;
  }
});
if (failed) {
  process.exit(1);
}

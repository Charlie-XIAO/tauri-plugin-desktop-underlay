import { quote } from "shell-quote";

const isWin = process.platform === "win32";

export default {
  "**/*.{js,jsx,ts,tsx,json,md,html,css,yml,yaml}": (filenames) => {
    const fileArgs = escape(filenames);
    return [`prettier --write ${fileArgs}`];
  },
  "**/*.rs": (filenames) => {
    const fileArgs = escape(filenames);
    return [`rustfmt -- ${fileArgs}`];
  },
};

function escape(filenames) {
  return filenames
    .map((filename) => (isWin ? `"${filename}"` : quote([filename])))
    .join(" ");
}

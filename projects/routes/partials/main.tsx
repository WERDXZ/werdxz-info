import { define } from "../../utils.ts";

export default define.page(function MainPartial() {
  // This route acts as a catch-all for partial requests
  // Fresh will automatically render just the <main> content
  // from the matched route without the full layout
  return null;
});

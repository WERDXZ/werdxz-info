export function formatBlogDate(dateString: string): string {
  try {
    return new Date(dateString).toLocaleDateString("en-US", {
      year: "numeric",
      month: "long",
      day: "numeric",
    });
  } catch (error) {
    console.error("[Date] Invalid date format:", dateString, error);
    return "Unknown date";
  }
}

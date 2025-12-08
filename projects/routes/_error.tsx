import { PageProps, HttpError } from "fresh";
import { Head } from "fresh/runtime";
import { define } from "../utils.ts";

export default define.page(function ErrorPage(props: PageProps) {
  const error = props.error;

  let title = "Error";
  let message = "An unexpected error occurred.";

  if (error instanceof HttpError) {
    if (error.status === 404) {
      title = "Not Found";
      message = error.message || "The page you're looking for doesn't exist.";
    } else {
      title = `Error ${error.status}`;
      message = error.message;
    }
  } else if (error instanceof Error) {
    message = error.message;
  }

  return (
    <>
      <Head>
        <title>{title} | werdxz</title>
      </Head>

      <div class="not-found">
        <h1>{title}</h1>
        <p>{message}</p>
        <a href="/" f-client-nav>&larr; Back to Projects</a>
      </div>
    </>
  );
});

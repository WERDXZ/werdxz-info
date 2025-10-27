import { PageProps } from "fresh";
import { Head } from "fresh/runtime";
import { define } from "../utils.ts";

export default define.layout(function Layout({ Component }: PageProps) {
  return (
    <>
      <Head>
        <link rel="stylesheet" href="https://cloud.werdxz.info/shared/styles/variables.css" />
        <link rel="stylesheet" href="/styles.css" />
      </Head>

      <header class="site-header">
        <nav class="site-nav" aria-label="Main navigation">
          <a href="/" class="site-title" f-client-nav>
            werdxz / projects
          </a>
          <div class="nav-links">
            <a href="https://werdxz.info" class="nav-link">
              Home
            </a>
            <a href="https://blog.werdxz.info" class="nav-link">
              Blog
            </a>
          </div>
        </nav>
      </header>

      <main class="main" f-partial="/partials/main">
        <Component />
      </main>

      <footer class="footer">
        <p>&copy; {new Date().getFullYear()} werdxz</p>
      </footer>
    </>
  );
});

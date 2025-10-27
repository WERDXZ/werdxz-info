import { component$, Slot } from "@builder.io/qwik";
import { Link } from "@builder.io/qwik-city";
import styles from "./layout.module.css";

export default component$(() => {
  return (
    <>
      <header class={styles.header}>
        <nav class={styles.nav} aria-label="Main navigation">
          <Link href="/" class={styles.logo}>
            werdxz / blog
          </Link>
          <div class={styles.navLinks}>
            <a href="https://werdxz.info" class={styles.navLink}>
              Home
            </a>
            <a href="https://projects.werdxz.info" class={styles.navLink}>
              Projects
            </a>
          </div>
        </nav>
      </header>

      <main class={styles.main}>
        <Slot />
      </main>

      <footer class={styles.footer}>
        <p>&copy; {new Date().getFullYear()} werdxz</p>
      </footer>
    </>
  );
});

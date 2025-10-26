import { $, component$, useSignal, useVisibleTask$ } from "@builder.io/qwik";
import { routeLoader$ } from "@builder.io/qwik-city";
import type { DocumentHead } from "@builder.io/qwik-city";
import { fetchPosts, fetchTags } from "~/lib/api";
import type { Post } from "~/lib/api";
import { PostCard } from "~/components/post-card/post-card";
import styles from "./index.module.css";

export const usePosts = routeLoader$(async ({ url }) => {
  const tags = url.searchParams.get('tags') || undefined;
  const search = url.searchParams.get('search') || undefined;

  try {
    const data = await fetchPosts({ limit: 10, page: 1, tags, search });
    return { ...data, filters: { tags, search } };
  } catch (error) {
    console.error("[SSR] Failed to fetch posts:", error);
    return {
      posts: [],
      pagination: {
        page: 1,
        limit: 10,
        total: 0,
        has_next: false,
      },
      filters: { tags, search },
    };
  }
});

export const useTags = routeLoader$(async () => {
  try {
    return await fetchTags();
  } catch (error) {
    console.error("[SSR] Failed to fetch tags:", error);
    return [];
  }
});

export default component$(() => {
  const initialData = usePosts();
  const availableTags = useTags();
  const allPosts = useSignal<Post[]>(initialData.value.posts);
  const currentPage = useSignal(1);
  const hasMore = useSignal(initialData.value.pagination.has_next);
  const isLoading = useSignal(false);
  const currentTags = useSignal<string | undefined>(initialData.value.filters.tags);
  const currentSearch = useSignal<string | undefined>(initialData.value.filters.search);
  const searchDebounceTimer = useSignal<number | undefined>(undefined);

  // eslint-disable-next-line qwik/no-use-visible-task
  useVisibleTask$(
    ({ track, cleanup }) => {
      track(() => allPosts.value);

      if (typeof window === 'undefined') return;

      // Handle browser back/forward navigation
      const handlePopState = () => {
        const url = new URL(window.location.href);
        const tags = url.searchParams.get('tags') || undefined;
        const search = url.searchParams.get('search') || undefined;

        // Update form controls to reflect URL state
        currentTags.value = tags;
        currentSearch.value = search;

        // Fetch posts with new filters
        applyFilters(tags, search);
      };

      window.addEventListener('popstate', handlePopState);
      cleanup(() => window.removeEventListener('popstate', handlePopState));

      // Create a sentinel element that triggers loading when visible
      const sentinel = document.createElement('div');
      sentinel.id = 'infinite-scroll-sentinel';
      sentinel.style.height = '1px';

      const loadMorePosts = async () => {
        // Prevent race conditions by checking and setting in one operation
        if (isLoading.value || !hasMore.value) return;
        isLoading.value = true;

        // Capture page at start to prevent using stale value
        const nextPage = currentPage.value + 1;

        try {
          const data = await fetchPosts({
            limit: 10,
            page: nextPage,
            tags: currentTags.value,
            search: currentSearch.value,
          });

          // Use concat instead of spread for better performance
          allPosts.value = allPosts.value.concat(data.posts);
          currentPage.value = nextPage;
          hasMore.value = data.pagination.has_next;
        } catch (error) {
          console.error('[Client] Failed to load more posts:', error);
        } finally {
          isLoading.value = false;
        }
      };

      const observer = new IntersectionObserver(
        (entries) => {
          if (entries[0].isIntersecting) {
            loadMorePosts();
          }
        },
        { rootMargin: '300px' }
      );

      // Insert sentinel before loading indicator
      const postsSection = document.querySelector(`.${styles.postsList}`);
      if (!postsSection?.parentElement) {
        // Cleanup if we can't find the posts section
        observer.disconnect();
        return;
      }

      postsSection.parentElement.insertBefore(sentinel, postsSection.nextSibling);
      observer.observe(sentinel);

      cleanup(() => {
        observer.disconnect();
        sentinel.remove();
      });
    },
    { strategy: 'document-ready' }
  );

  const applyFilters = $(async (tags?: string, search?: string) => {
    // Update URL
    const url = new URL(window.location.href);
    if (tags) {
      url.searchParams.set('tags', tags);
    } else {
      url.searchParams.delete('tags');
    }
    if (search) {
      url.searchParams.set('search', search);
    } else {
      url.searchParams.delete('search');
    }
    window.history.pushState({}, '', url);

    // Reset state and fetch from page 1
    isLoading.value = true;
    currentTags.value = tags;
    currentSearch.value = search;

    try {
      const data = await fetchPosts({ limit: 10, page: 1, tags, search });
      allPosts.value = data.posts;
      currentPage.value = 1;
      hasMore.value = data.pagination.has_next;
    } catch (error) {
      console.error('[Client] Failed to apply filters:', error);
    } finally {
      isLoading.value = false;
    }
  });

  return (
    <>
      <section class={styles.filtersSection}>
        <form class={styles.filters} preventdefault:submit>
          <input
            type="search"
            placeholder="Search posts..."
            value={currentSearch.value || ''}
            onInput$={(e) => {
              const value = (e.target as HTMLInputElement).value;
              currentSearch.value = value || undefined;

              // Clear existing timeout
              if (searchDebounceTimer.value !== undefined) {
                clearTimeout(searchDebounceTimer.value);
              }

              // Set new timeout
              searchDebounceTimer.value = setTimeout(() => {
                applyFilters(currentTags.value, value || undefined);
              }, 300) as unknown as number;
            }}
            class={styles.searchInput}
            aria-label="Search posts"
          />

          <label for="tag-select" class={styles.tagLabel}>Filter by tag:</label>
          <select
            id="tag-select"
            class={styles.tagSelect}
            value={currentTags.value || ''}
            onChange$={(e) => {
              const value = (e.target as HTMLSelectElement).value;
              applyFilters(value || undefined, currentSearch.value);
            }}
          >
            <option value="">All tags</option>
            {availableTags.value.map((tagData) => (
              <option key={tagData.tag} value={tagData.tag}>
                {`${tagData.tag} (${tagData.count})`}
              </option>
            ))}
          </select>
        </form>
      </section>

      <section class={styles.postsSection}>
        {allPosts.value.length === 0 ? (
          <p class={styles.emptyState}>No posts yet. Check back soon!</p>
        ) : (
          <ul class={styles.postsList}>
            {allPosts.value.map((post) => (
              <li key={post.slug}>
                <PostCard post={post} />
              </li>
            ))}
          </ul>
        )}
      </section>

      {isLoading.value && (
        <section class={styles.loadingSection}>
          <p class={styles.loadingIndicator}>Loading more posts...</p>
        </section>
      )}

      {!hasMore.value && allPosts.value.length > 0 && (
        <section class={styles.endSection}>
          <p class={styles.endMessage}>You've reached the end!</p>
        </section>
      )}
    </>
  );
});

export const head: DocumentHead = {
  title: "Blog | werdxz.info",
};

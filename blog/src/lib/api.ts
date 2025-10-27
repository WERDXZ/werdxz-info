import type { Blog, BlogsResponse, TagWithCount } from './types';

const API_BASE_URL = import.meta.env.PUBLIC_API_BASE_URL || 'https://api.werdxz.info/v1';
const FETCH_TIMEOUT = 10000; // 10 seconds

export type { Blog, TagWithCount };

async function fetchWithTimeout(url: string, timeout = FETCH_TIMEOUT): Promise<Response> {
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), timeout);

  try {
    const response = await fetch(url, { signal: controller.signal });
    clearTimeout(timeoutId);
    return response;
  } catch (error) {
    clearTimeout(timeoutId);
    if (error instanceof Error && error.name === 'AbortError') {
      throw new Error(`Request timeout after ${timeout}ms`);
    }
    throw error;
  }
}

export interface FetchBlogsOptions {
  limit?: number;
  page?: number;
  tags?: string;
  search?: string;
}

export async function fetchBlogs({
  limit = 10,
  page = 1,
  tags,
  search,
}: FetchBlogsOptions = {}): Promise<BlogsResponse> {
  const params = new URLSearchParams({
    limit: limit.toString(),
    page: page.toString(),
  });

  if (tags) params.set('tags', tags);
  if (search) params.set('search', search);

  const url = `${API_BASE_URL}/blogs?${params}`;
  const response = await fetchWithTimeout(url);

  if (!response.ok) {
    throw new Error(`Failed to fetch blogs: ${response.statusText}`);
  }

  return response.json();
}

export async function fetchBlog(slug: string): Promise<Blog> {
  const url = `${API_BASE_URL}/blogs/${slug}`;
  const response = await fetchWithTimeout(url);

  if (!response.ok) {
    throw new Error(`Failed to fetch blog: ${response.statusText}`);
  }

  return response.json();
}

export async function fetchTags(): Promise<TagWithCount[]> {
  const url = `${API_BASE_URL}/tags`;
  const response = await fetchWithTimeout(url);

  if (!response.ok) {
    throw new Error(`Failed to fetch tags: ${response.statusText}`);
  }

  return response.json();
}

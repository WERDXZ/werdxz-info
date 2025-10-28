export interface Post {
  slug: string;
  title: string;
  summary: string;
  content?: string;
  published_at: string;
  tags?: string[];
  read_time_minutes?: number;
}

export interface PostsResponse {
  posts: Post[];
  pagination: {
    page: number;
    limit: number;
    total: number;
    has_next: boolean;
  };
}

export interface TagWithCount {
  tag: string;
  count: number;
}

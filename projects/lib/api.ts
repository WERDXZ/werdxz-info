// API client for fetching projects data
export interface ProjectUrl {
  label: string;
  url: string;
}

export interface Project {
  id: string;
  slug: string;
  name: string;
  description: string;
  stage: "planned" | "wip" | "active" | "maintained" | "archived" | "shelved";
  open_to_contributors: boolean;
  readme_url: string;
  tags: string[];
  urls: ProjectUrl[];
  created_at: string;
  updated_at: string;
}

export interface ProjectsResponse {
  projects: Project[];
}

const API_BASE_URL = "https://api.werdxz.info/v1";
const FETCH_TIMEOUT = 10000; // 10 seconds

async function fetchWithTimeout(url: string, timeout = FETCH_TIMEOUT): Promise<Response> {
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), timeout);

  try {
    const response = await fetch(url, { signal: controller.signal });
    clearTimeout(timeoutId);
    return response;
  } catch (error) {
    clearTimeout(timeoutId);
    if (error instanceof Error && error.name === "AbortError") {
      throw new Error(`Request timeout after ${timeout}ms`);
    }
    throw error;
  }
}

export async function getProjects(): Promise<Project[]> {
  const url = `${API_BASE_URL}/projects`;
  const response = await fetchWithTimeout(url);

  if (!response.ok) {
    throw new Error(`Failed to fetch projects: ${response.statusText}`);
  }

  const data: ProjectsResponse = await response.json();
  return data.projects;
}

export async function getProjectBySlug(slug: string): Promise<Project | undefined> {
  const url = `${API_BASE_URL}/projects/${slug}`;

  try {
    const response = await fetchWithTimeout(url);

    if (!response.ok) {
      if (response.status === 404) {
        return undefined;
      }
      throw new Error(`Failed to fetch project: ${response.statusText}`);
    }

    return await response.json();
  } catch (error) {
    console.error(`Error fetching project ${slug}:`, error);
    return undefined;
  }
}

export async function fetchReadme(url: string): Promise<string> {
  try {
    const response = await fetchWithTimeout(url);
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return await response.text();
  } catch (error) {
    console.error(`Failed to fetch README from ${url}:`, error);
    return `# README Unavailable

Unable to load the project README at this time. Please visit the project repository directly for documentation.`;
  }
}

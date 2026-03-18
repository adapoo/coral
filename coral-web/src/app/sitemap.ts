import type { MetadataRoute } from "next";
import { getTopPopularPlayers } from "@/lib/utils/server/popular";

export default async function sitemap(): Promise<MetadataRoute.Sitemap> {
  const base = process.env.INTERNAL_BASE_URL || "https://coral.urchin.ws";
  const urls: MetadataRoute.Sitemap = [
    {
      url: base,
      lastModified: new Date(),
      changeFrequency: "daily",
      priority: 1,
    },
  ];
  try {
    const top = await getTopPopularPlayers(200);
    top.forEach((p) => {
      const slug = encodeURIComponent(p.uuid);
      urls.push({
        url: `${base}/player/${slug}`,
        lastModified: new Date(),
        changeFrequency: "weekly",
        priority: 0.6,
      });
    });
  } catch {}
  return urls;
}

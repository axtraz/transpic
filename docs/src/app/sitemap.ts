import type { MetadataRoute } from "next";

export default function sitemap(): MetadataRoute.Sitemap {
    return [
        {
            url: "/",
            priority: 1,
        },
        {
            url: "/docs",
            priority: 0.9,
        },
    ];
}

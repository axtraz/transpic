import { DocsLayout } from "fumadocs-ui/layouts/docs";
import { baseOptions } from "@/lib/layout.shared";
import { source } from "@/lib/source";

export default function Layout({ children }: LayoutProps<"/docs">) {
    const base = baseOptions();
    return (
        <DocsLayout
            tree={source.pageTree}
            {...base}
            nav={{
                ...base.nav,
                transparentMode: "always",
            }}
        >
            {children}
        </DocsLayout>
    );
}

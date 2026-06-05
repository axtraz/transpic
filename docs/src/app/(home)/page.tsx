"use client";

import {
    Zap,
    ImageIcon,
    Terminal,
    Copy,
    Check,
    RotateCw,
    Contrast,
    ScanLine,
    Maximize2,
    Blend,
    RefreshCw,
} from "lucide-react";
import { motion, useInView } from "motion/react";
import Link from "next/link";
import { useState, useRef } from "react";
import { Button } from "@/components/ui/button";

const links = [
    { label: "GitHub", href: "https://github.com/axtraz/transpic" },
    { label: "npm", href: "/" },
    { label: "Docs", href: "/docs" },
];

const features = [
    {
        icon: RefreshCw,
        title: "Format Conversion",
        description: "PNG, JPG, WebP, AVIF, GIF, TIFF, BMP and more.",
        tag: "--format <format>",
    },
    {
        icon: Maximize2,
        title: "Resize",
        description: "High-quality resampling to any dimension.",
        tag: "--resize <width>x<height>",
    },
    {
        icon: Blend,
        title: "Blur",
        description: "Gaussian blur with configurable intensity.",
        tag: "--blur <intensity>",
    },
    { icon: ScanLine, title: "Grayscale", description: "Convert to grayscale in milliseconds.", tag: "--grayscale" },
    { icon: Contrast, title: "Invert", description: "Invert colors for creative workflows.", tag: "--invert" },
    { icon: RotateCw, title: "Rotate", description: "Rotate by 90, 180, or 270 degrees.", tag: "--rotate <degrees>" },
];

const examples = [
    { label: "Convert format", command: "transpic --path photo.jpg --format webp" },
    { label: "Resize image", command: "transpic --path photo.png --resize 800x600" },
    { label: "Apply blur", command: "transpic --path photo.jpg --blur 2 --grayscale" },
    { label: "Chain everything", command: "transpic --path photo.png --format webp --resize 1280x720 --grayscale" },
];

const stats = [
    { value: "~10ms", label: "JPG → PNG" },
    { value: "~4x", label: "vs ImageMagick" },
    { value: "~2MB", label: "binary size" },
];

const fadeUp = {
    hidden: { opacity: 0, y: 24 },
    visible: { opacity: 1, y: 0 },
};

const stagger = {
    visible: { transition: { staggerChildren: 0.1 } },
};

function useCopy(text: string) {
    const [copied, setCopied] = useState(false);
    const copy = () => {
        navigator.clipboard.writeText(text);
        setCopied(true);
        setTimeout(() => setCopied(false), 2000);
    };
    return { copied, copy };
}

function CopyBtn({ text, className = "" }: { text: string; className?: string }) {
    const { copied, copy } = useCopy(text);
    return (
        <button onClick={copy} aria-label="Copy" className={`transition-colors ${className}`}>
            {copied ? (
                <Check size={13} className="text-green-400" />
            ) : (
                <Copy size={13} className="text-zinc-500 hover:text-zinc-300" />
            )}
        </button>
    );
}

function InstallBlock() {
    const cmd = "npm install -g transpic-cli";
    const { copied, copy } = useCopy(cmd);
    return (
        <button
            onClick={copy}
            className="group inline-flex w-full items-center gap-3 rounded-2xl border border-zinc-700/80 bg-zinc-900 px-5 py-3.5 font-mono text-sm text-zinc-200 transition-all duration-200 hover:border-zinc-600 hover:bg-zinc-800 sm:w-auto"
        >
            <span className="text-orange-400 select-none">$</span>
            <span className="flex-1 text-left">{cmd}</span>
            <span className="ml-2 text-zinc-500 transition-colors group-hover:text-zinc-300">
                {copied ? <Check size={14} className="text-green-400" /> : <Copy size={14} />}
            </span>
        </button>
    );
}

function AnimatedSection({ children, className = "" }: { children: React.ReactNode; className?: string }) {
    const ref = useRef(null);
    const inView = useInView(ref, { once: true, margin: "-80px" });
    return (
        <motion.div
            ref={ref}
            variants={fadeUp}
            initial="hidden"
            animate={inView ? "visible" : "hidden"}
            transition={{ duration: 0.5, ease: "easeOut" }}
            className={className}
        >
            {children}
        </motion.div>
    );
}

export default function TranspicLanding() {
    const [activeExample, setActiveExample] = useState(0);

    return (
        <div className="min-h-screen bg-zinc-950 text-zinc-100 antialiased selection:bg-orange-500/30">
            <section className="relative overflow-hidden">
                <div className="absolute inset-0 bg-[linear-gradient(to_right,#ffffff06_1px,transparent_1px),linear-gradient(to_bottom,#ffffff06_1px,transparent_1px)] [mask-image:linear-gradient(to_bottom,black_30%,transparent_100%)] bg-[size:48px_48px]" />
                <div className="absolute inset-0 bg-[radial-gradient(ellipse_80%_50%_at_50%_-10%,#f97316_08,transparent_60%)]" />

                <motion.div
                    className="relative mx-auto max-w-6xl px-4 pt-20 pb-16 text-center sm:px-6 sm:pt-28 sm:pb-20"
                    variants={stagger}
                    initial="hidden"
                    animate="visible"
                >
                    <motion.div
                        variants={fadeUp}
                        transition={{ duration: 0.5, ease: "easeOut" }}
                        className="mb-8 inline-flex items-center gap-2 rounded-full border border-orange-500/25 bg-orange-500/10 px-3 py-1.5 text-xs font-medium text-orange-300"
                    >
                        <Zap size={11} className="text-orange-400" />
                        ~4x faster than ImageMagick
                    </motion.div>

                    <motion.h1
                        variants={fadeUp}
                        transition={{ duration: 0.6, ease: "easeOut" }}
                        className="mb-6 px-2 text-4xl leading-[1.08] font-bold tracking-tight text-white sm:mb-8 sm:text-5xl lg:text-7xl"
                    >
                        Image manipulation, <br className="hidden sm:block" />
                        <span className="bg-gradient-to-r from-orange-400 to-orange-600 bg-clip-text text-transparent">
                            without the overhead.
                        </span>
                    </motion.h1>

                    <motion.p
                        variants={fadeUp}
                        transition={{ duration: 0.5, ease: "easeOut" }}
                        className="mx-auto mb-10 max-w-lg px-4 text-base leading-relaxed text-zinc-400 sm:text-lg"
                    >
                        A blazing-fast CLI written in Rust. Convert, resize, blur, rotate, and filter images — installed
                        in seconds with npm.
                    </motion.p>

                    <motion.div
                        variants={fadeUp}
                        transition={{ duration: 0.5, ease: "easeOut" }}
                        className="mb-10 flex flex-col items-center justify-center gap-3 px-4 sm:flex-row"
                    >
                        <InstallBlock />
                        <Button
                            variant="ghost"
                            className="h-12 w-full gap-2 rounded-xl border border-zinc-800 px-5 text-sm text-zinc-400 hover:border-zinc-700 hover:bg-zinc-800 hover:text-white sm:w-auto"
                        >
                            <Terminal size={14} />
                            <Link href="/docs">Read the docs</Link>
                        </Button>
                    </motion.div>

                    <motion.div
                        variants={fadeUp}
                        transition={{ duration: 0.5, ease: "easeOut" }}
                        className="flex flex-wrap items-center justify-center gap-4 px-4 text-xs text-zinc-500 sm:gap-6"
                    >
                        {[
                            { color: "bg-green-400", label: "Written in Rust" },
                            { color: "bg-orange-400", label: "Zero system dependencies" },
                            { color: "bg-blue-400", label: "Open source · MIT" },
                        ].map(({ color, label }) => (
                            <span key={label} className="flex items-center gap-1.5">
                                <span className={`h-1.5 w-1.5 rounded-full ${color}`} />
                                {label}
                            </span>
                        ))}
                    </motion.div>
                </motion.div>
            </section>

            <AnimatedSection className="mx-auto max-w-3xl px-4 pb-20 sm:px-6 sm:pb-24">
                <div className="overflow-hidden rounded-2xl border border-zinc-800 bg-zinc-900/60 backdrop-blur-sm">
                    <div className="flex items-center gap-2 border-b border-zinc-800 bg-zinc-900/80 px-4 py-3">
                        <span className="h-3 w-3 rounded-full bg-[#ff5f57]" />
                        <span className="h-3 w-3 rounded-full bg-[#ffbd2e]" />
                        <span className="h-3 w-3 rounded-full bg-[#28ca41]" />
                        <span className="ml-3 font-mono text-xs tracking-wide text-zinc-500">transpic-cli</span>
                    </div>

                    <div className="flex scrollbar-none overflow-x-auto border-b border-zinc-800">
                        {examples.map((ex, i) => (
                            <button
                                key={i}
                                onClick={() => setActiveExample(i)}
                                className={`flex-shrink-0 border-b-2 px-3 py-2.5 font-mono text-xs whitespace-nowrap transition-colors sm:px-4 ${
                                    i === activeExample
                                        ? "border-orange-500 bg-orange-500/5 text-orange-400"
                                        : "border-transparent text-zinc-500 hover:bg-zinc-800/50 hover:text-zinc-300"
                                }`}
                            >
                                {ex.label}
                            </button>
                        ))}
                    </div>

                    <div className="min-h-[100px] p-4 font-mono text-sm sm:p-6">
                        {examples.map((ex, i) => (
                            <div
                                key={i}
                                className={`transition-all duration-300 ${i === activeExample ? "block" : "hidden"}`}
                            >
                                <div className="flex items-start gap-3">
                                    <span className="mt-0.5 flex-shrink-0 text-orange-400">$</span>
                                    <div className="min-w-0 flex-1">
                                        <span className="break-all text-zinc-200">{ex.command}</span>
                                        <div className="mt-3 flex items-center gap-2 border-t border-zinc-800 pt-3">
                                            <span className="text-xs text-green-400">
                                                ✓ Image edited in &lt;number&gt;ms
                                            </span>
                                        </div>
                                    </div>
                                    <CopyBtn text={ex.command} className="mt-0.5 flex-shrink-0" />
                                </div>
                            </div>
                        ))}
                    </div>
                </div>
            </AnimatedSection>

            <AnimatedSection className="mb-20 border-y border-zinc-800/60 bg-zinc-900/20 py-12 sm:mb-24 sm:py-16">
                <div className="mx-auto max-w-6xl px-4 sm:px-6">
                    <div className="grid grid-cols-3 gap-4 text-center sm:gap-8">
                        {stats.map(({ value, label }, i) => (
                            <motion.div
                                key={label}
                                initial={{ opacity: 0, y: 16 }}
                                whileInView={{ opacity: 1, y: 0 }}
                                viewport={{ once: true }}
                                transition={{ duration: 0.4, delay: i * 0.1, ease: "easeOut" }}
                                className="space-y-1"
                            >
                                <div className="text-2xl font-bold tracking-tight text-white tabular-nums sm:text-4xl lg:text-5xl">
                                    {value}
                                </div>
                                <div className="text-xs text-zinc-500 sm:text-sm">{label}</div>
                            </motion.div>
                        ))}
                    </div>
                </div>
            </AnimatedSection>

            <section className="mx-auto max-w-6xl px-4 pb-20 sm:px-6 sm:pb-28">
                <AnimatedSection className="mb-10 text-center sm:mb-14">
                    <h2 className="mb-3 text-2xl font-bold text-white sm:text-3xl">Everything you need</h2>
                    <p className="mx-auto max-w-sm text-sm text-zinc-400 sm:text-base">
                        The most common image operations, without the complexity.
                    </p>
                </AnimatedSection>

                <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 sm:gap-4 lg:grid-cols-3">
                    {features.map(({ icon: Icon, title, description, tag }, i) => (
                        <motion.div
                            key={title}
                            initial={{ opacity: 0, y: 24 }}
                            whileInView={{ opacity: 1, y: 0 }}
                            viewport={{ once: true, margin: "-40px" }}
                            transition={{ duration: 0.4, delay: i * 0.07, ease: "easeOut" }}
                            whileHover={{ y: -2 }}
                            className="group relative overflow-hidden rounded-2xl border border-zinc-800 bg-zinc-900/40 p-5 transition-colors duration-200 hover:border-zinc-700 hover:bg-zinc-900/80 sm:p-6"
                        >
                            <div className="absolute top-0 right-0 h-24 w-24 translate-x-8 -translate-y-8 rounded-full bg-orange-500/3 transition-colors group-hover:bg-orange-500/6" />
                            <div className="mb-4 flex h-9 w-9 items-center justify-center rounded-xl border border-orange-500/20 bg-orange-500/10">
                                <Icon size={16} className="text-orange-400" />
                            </div>
                            <h3 className="mb-1.5 text-sm font-semibold text-white">{title}</h3>
                            <p className="mb-4 text-sm leading-relaxed text-zinc-400">{description}</p>
                            <code className="rounded-lg border border-orange-500/15 bg-orange-500/8 px-2.5 py-1 font-mono text-xs text-orange-300/80">
                                {tag}
                            </code>
                        </motion.div>
                    ))}
                </div>
            </section>

            <section className="relative overflow-hidden border-t border-zinc-800/60 py-20 sm:py-28">
                <div className="absolute inset-0 bg-[radial-gradient(ellipse_60%_80%_at_50%_110%,#f97316_06,transparent_70%)]" />
                <AnimatedSection className="relative mx-auto max-w-6xl px-4 text-center sm:px-6">
                    <h2 className="mb-4 text-3xl font-bold text-white sm:text-4xl lg:text-5xl">Start in 10 seconds.</h2>
                    <p className="mb-10 text-sm text-zinc-400 sm:text-base">
                        No system dependencies. No config. No friction.
                    </p>
                    <div className="flex flex-col items-center justify-center gap-3 px-4 sm:flex-row">
                        <InstallBlock />
                        <Button
                            variant="ghost"
                            className="h-12 w-full gap-2 rounded-xl border border-zinc-800 px-5 text-sm text-zinc-400 hover:border-zinc-700 hover:bg-zinc-800 hover:text-white sm:w-auto"
                        >
                            <Terminal size={14} />
                            <Link href="/docs">Read the docs</Link>
                        </Button>
                    </div>
                </AnimatedSection>
            </section>

            <footer className="border-t border-zinc-800/60 px-4 py-8 sm:px-6">
                <div className="mx-auto flex max-w-6xl flex-col items-center justify-between gap-4 text-xs text-zinc-600 sm:flex-row">
                    <div className="flex items-center gap-2">
                        <ImageIcon size={13} className="text-orange-400/60" />
                        <span>
                            transpic-cli —{" "}
                            <a
                                href="https://github.com/axtraz/transpic/blob/main/LICENSE"
                                className="transition-colors hover:text-zinc-300"
                            >
                                MIT License
                            </a>
                        </span>
                    </div>
                    <div className="flex items-center gap-5">
                        {links.map(link => (
                            <a key={link.label} href={link.href} className="transition-colors hover:text-zinc-300">
                                {link.label}
                            </a>
                        ))}
                    </div>
                </div>
            </footer>
        </div>
    );
}

import { useMemo, useRef, useState, type ComponentType } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  Aperture,
  Sun,
  Contrast,
  Palette,
  SunMoon,
  Grid3x3,
  Maximize2,
  RotateCw,
  Upload,
  Image as ImageIcon,
  Copy,
  Check,
  Loader2,
  X,
  Download,
  ChevronDown,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Slider } from "@/components/ui/slider";
import { Switch } from "@/components/ui/switch";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import { cn } from "@/lib/utils";
import "./App.css";

type ParamType = "range" | "number" | "boolean";

interface ParamSchema {
  key: string;
  label: string;
  type: ParamType;
  min: number;
  max: number;
  step: number;
  default: number;
  unit?: string;
}

interface Operation {
  id: string;
  label: string;
  cliFlag: string;
  params: ParamSchema[];
  icon: ComponentType<{ className?: string }>;
}

const OPERATIONS: Operation[] = [
  {
    id: "blur",
    label: "Blur",
    cliFlag: "--blur",
    icon: Aperture,
    params: [{ key: "radius", label: "Radius", type: "range", min: 0, max: 50, step: 1, default: 8, unit: "px" }],
  },
  {
    id: "brighten",
    label: "Brighten",
    cliFlag: "--brighten",
    icon: Sun,
    params: [{ key: "amount", label: "Amount", type: "range", min: -100, max: 100, step: 1, default: 20 }],
  },
  {
    id: "grayscale",
    label: "Grayscale",
    cliFlag: "--grayscale",
    icon: Contrast,
    params: [{ key: "enabled", label: "Apply", type: "boolean", min: 0, max: 1, step: 1, default: 0 }],
  },
  {
    id: "huerotate",
    label: "Hue rotate",
    cliFlag: "--hue-rotate",
    icon: Palette,
    params: [{ key: "degrees", label: "Degrees", type: "range", min: 0, max: 360, step: 1, default: 90, unit: "°" }],
  },
  {
    id: "invert",
    label: "Invert",
    cliFlag: "--invert",
    icon: SunMoon,
    params: [{ key: "enabled", label: "Apply", type: "boolean", min: 0, max: 1, step: 1, default: 0 }],
  },
  {
    id: "pixelate",
    label: "Pixelate",
    cliFlag: "--pixelate",
    icon: Grid3x3,
    params: [{ key: "blockSize", label: "Block size", type: "range", min: 1, max: 50, step: 1, default: 8, unit: "px" }],
  },
  {
    id: "resize",
    label: "Resize",
    cliFlag: "--resize",
    icon: Maximize2,
    params: [
      { key: "width", label: "Width", type: "number", min: 1, max: 8000, step: 1, default: 1024, unit: "px" },
      { key: "height", label: "Height", type: "number", min: 1, max: 8000, step: 1, default: 1024, unit: "px" },
    ],
  },
  {
    id: "rotate",
    label: "Rotate",
    cliFlag: "--rotate",
    icon: RotateCw,
    params: [{ key: "degrees", label: "Degrees", type: "range", min: 0, max: 359, step: 1, default: 90, unit: "°" }],
  },
];

const FORMATS = [
  "png",
  "jpeg",
  "webp",
  "avif",
  "bmp",
  "gif",
  "hdr",
  "ico",
  "pnm",
  "qoi",
  "tga",
  "tiff",
  "exr",
  "ff",
] as const;

function App() {
  const [enabledOps, setEnabledOps] = useState<Record<string, boolean>>(
    Object.fromEntries(OPERATIONS.map((op) => [op.id, false]))
  );
  const [expandedId, setExpandedId] = useState<string | null>(null);
  const [paramValues, setParamValues] = useState<Record<string, Record<string, number>>>(
    Object.fromEntries(
      OPERATIONS.map((op) => [op.id, Object.fromEntries(op.params.map((p) => [p.key, p.default]))])
    )
  );
  const [format, setFormat] = useState<(typeof FORMATS)[number]>("webp");
  const [isDragging, setIsDragging] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
  const [copied, setCopied] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [imageUrl, setImageUrl] = useState<string | null>(null);
  const [processedUrl, setProcessedUrl] = useState<string | null>(null);
  const [previewMode, setPreviewMode] = useState<"original" | "result">("original");
  const [imageName, setImageName] = useState<string | null>(null);
  const [imageDims, setImageDims] = useState<{ w: number; h: number } | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const loadFile = (file: File) => {
    const reader = new FileReader();
    reader.onload = () => {
      const dataUrl = reader.result as string;
      setImageUrl(dataUrl);
      setImageName(file.name);
      setProcessedUrl(null);
      setPreviewMode("original");
      setError(null);
      const img = new Image();
      img.onload = () => setImageDims({ w: img.width, h: img.height });
      img.src = dataUrl;
    };
    reader.readAsDataURL(file);
  };

  const clearImage = () => {
    setImageUrl(null);
    setProcessedUrl(null);
    setImageName(null);
    setImageDims(null);
    setError(null);
  };

  const onDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);
    const file = e.dataTransfer.files?.[0];
    if (file) loadFile(file);
  };

  const setParam = (opId: string, key: string, value: number) => {
    setParamValues((prev) => ({ ...prev, [opId]: { ...prev[opId], [key]: value } }));
  };

  const toggleEnabled = (id: string, checked: boolean) => {
    setEnabledOps((prev) => ({ ...prev, [id]: checked }));
    if (checked) setExpandedId(id);
  };

  const handleRowClick = (op: Operation) => {
    if (!enabledOps[op.id]) {
      toggleEnabled(op.id, true);
    } else if (op.params.length > 0) {
      setExpandedId((cur) => (cur === op.id ? null : op.id));
    }
  };

  const activeOps = useMemo(() => OPERATIONS.filter((op) => enabledOps[op.id]), [enabledOps]);

  const cliPreview = useMemo(() => {
    const base = imageName ?? "input.png";
    const parts = activeOps.map((op) => {
      if (op.params.length === 0) return op.cliFlag;
      return op.params
        .map((p) => {
          if (p.type === "boolean") {
            return paramValues[op.id][p.key] === 1 ? op.cliFlag : "";
          }
          return `${op.cliFlag} ${paramValues[op.id][p.key]}`;
        })
        .filter(Boolean)
        .join(" ");
    });
    const flags = parts.filter(Boolean).length ? " " + parts.filter(Boolean).join(" ") : "";
    const out = `output.${format}`;
    return `transpic ${base}${flags} -f ${format} -o ${out}`;
  }, [imageName, activeOps, paramValues, format]);

  const copyCommand = async () => {
    await navigator.clipboard.writeText(cliPreview);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  };

  const processImage = async () => {
    if (!imageUrl) return;
    if (activeOps.length === 0) {
      setError("Enable at least one operation");
      return;
    }
    setIsProcessing(true);
    setError(null);
    try {
      const result = await invoke<string>("process_image", {
        imageData: imageUrl,
        operations: activeOps.map((op) => ({ id: op.id, params: paramValues[op.id] })),
        format,
      });
      setProcessedUrl(result);
      setPreviewMode("result");
    } catch (err) {
      setError(String(err));
    } finally {
      setIsProcessing(false);
    }
  };

  const downloadResult = () => {
    if (!processedUrl) return;
    const a = document.createElement("a");
    a.href = processedUrl;
    a.download = `${imageName?.split(".")[0] ?? "transpic-output"}.${format}`;
    a.click();
  };

  const displayedUrl = previewMode === "result" && processedUrl ? processedUrl : imageUrl;

  return (
    <main className="flex h-full w-full bg-background text-foreground font-sans">
      <aside className="w-[320px] shrink-0 border-r border-border bg-card flex flex-col overflow-y-auto">
        <div className="px-5 py-4 border-b border-border">
          <h1 className="text-sm font-medium tracking-tight">transpic</h1>
          <p className="text-xs text-muted-foreground mt-0.5">image processing</p>
        </div>

        {/* File picker */}
        <div className="px-4 pt-4">
          <input
            ref={fileInputRef}
            type="file"
            accept="image/*"
            className="hidden"
            onChange={(e) => {
              const file = e.target.files?.[0];
              if (file) loadFile(file);
            }}
          />
          <button
            onClick={() => fileInputRef.current?.click()}
            onDragOver={(e) => {
              e.preventDefault();
              setIsDragging(true);
            }}
            onDragLeave={() => setIsDragging(false)}
            onDrop={onDrop}
            className={cn(
              "w-full rounded-md border border-dashed px-3 py-3 text-xs text-left transition-colors flex items-center gap-2.5",
              isDragging ? "border-primary bg-primary/5" : "border-input hover:border-muted-foreground"
            )}
          >
            <Upload className="w-3.5 h-3.5 text-muted-foreground shrink-0" />
            <div className="min-w-0">
              <div className="font-medium truncate">{imageName ?? "Drop an image or click to browse"}</div>
              {imageDims && (
                <div className="text-muted-foreground font-mono mt-0.5">
                  {imageDims.w} × {imageDims.h}
                </div>
              )}
            </div>
          </button>
        </div>

        <div className="px-4 pt-5">
          <div className="text-[10px] uppercase tracking-wide text-muted-foreground mb-2 px-1">
            Operations · {activeOps.length} active
          </div>
          <div className="space-y-0.5">
            {OPERATIONS.map((op) => {
              const Icon = op.icon;
              const enabled = enabledOps[op.id];
              const expanded = expandedId === op.id;
              const hasParams = op.params.length > 0;
              return (
                <div key={op.id} className="rounded-md overflow-hidden">
                  <div
                    className={cn(
                      "flex items-center gap-2.5 px-3 py-2 rounded-md text-sm transition-colors",
                      enabled ? "bg-secondary text-foreground" : "text-muted-foreground hover:bg-secondary/50"
                    )}
                  >
                    <Switch
                      checked={enabled}
                      onCheckedChange={(checked) => toggleEnabled(op.id, checked)}
                      className="shrink-0"
                    />
                    <button
                      onClick={() => handleRowClick(op)}
                      className="flex items-center gap-2.5 flex-1 min-w-0 text-left"
                    >
                      <Icon className={cn("w-4 h-4 shrink-0", enabled && "text-primary")} />
                      <span className="truncate">{op.label}</span>
                    </button>
                    {hasParams && (
                      <ChevronDown
                        className={cn(
                          "w-3.5 h-3.5 text-muted-foreground transition-transform shrink-0",
                          expanded && "rotate-180"
                        )}
                      />
                    )}
                  </div>
                  {hasParams && expanded && (
                    <div className="px-3 pb-3 pt-1 space-y-3">
                      {op.params.map((p) => (
                        <div key={p.key}>
                          <div className="flex justify-between text-xs mb-1.5">
                            <span className="text-muted-foreground">{p.label}</span>
                            {p.type !== "boolean" && (
                              <span className="font-mono text-primary">
                                {paramValues[op.id][p.key]}
                                {p.unit ?? ""}
                              </span>
                            )}
                          </div>
                          {p.type === "range" && (
                            <Slider
                              min={p.min}
                              max={p.max}
                              step={p.step}
                              value={[paramValues[op.id][p.key]]}
                              onValueChange={([v]) => setParam(op.id, p.key, v)}
                            />
                          )}
                          {p.type === "number" && (
                            <input
                              type="number"
                              min={p.min}
                              max={p.max}
                              step={p.step}
                              value={paramValues[op.id][p.key]}
                              onChange={(e) => setParam(op.id, p.key, Number(e.target.value))}
                              className="w-full bg-background border border-input rounded px-2 py-1.5 text-sm font-mono"
                            />
                          )}
                          {p.type === "boolean" && (
                            <ToggleGroup
                              type="single"
                              value={paramValues[op.id][p.key] === 1 ? "yes" : "no"}
                              onValueChange={(v) => v && setParam(op.id, p.key, v === "yes" ? 1 : 0)}
                              className="grid grid-cols-2"
                            >
                              <ToggleGroupItem value="yes">Yes</ToggleGroupItem>
                              <ToggleGroupItem value="no">No</ToggleGroupItem>
                            </ToggleGroup>
                          )}
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </div>

        <div className="px-5 pt-5 mt-5 border-t border-border space-y-3">
          <div className="text-[10px] uppercase tracking-wide text-muted-foreground pt-1">Output format</div>
          <select
            value={format}
            onChange={(e) => setFormat(e.target.value as (typeof FORMATS)[number])}
            className="w-full bg-secondary border border-input rounded px-2 py-1.5 text-sm font-mono"
          >
            {FORMATS.map((f) => (
              <option key={f} value={f}>
                {f}
              </option>
            ))}
          </select>
        </div>

        <div className="flex-1" />

        {error && (
          <div className="mx-4 mb-3 px-3 py-2 rounded-md border border-destructive/30 bg-destructive/10 text-xs text-destructive">
            {error}
          </div>
        )}

        <div className="px-4 pb-3">
          <button
            onClick={copyCommand}
            className="w-full group bg-background border border-border rounded-md px-3 py-2 flex items-center gap-2 hover:border-input transition-colors"
          >
            <span className="font-mono text-[10px] text-muted-foreground truncate flex-1 text-left">
              $ {cliPreview}
            </span>
            {copied ? (
              <Check className="w-3 h-3 text-primary shrink-0" />
            ) : (
              <Copy className="w-3 h-3 text-muted-foreground shrink-0" />
            )}
          </button>
        </div>

        <div className="px-4 pb-4 space-y-2">
          <Button onClick={processImage} disabled={!imageUrl || isProcessing} className="w-full">
            {isProcessing && <Loader2 className="w-3.5 h-3.5 animate-spin" />}
            {isProcessing ? "Processing…" : "Process image"}
          </Button>
          <Button onClick={downloadResult} disabled={!processedUrl} variant="secondary" className="w-full">
            <Download className="w-3.5 h-3.5" />
            Download
          </Button>
        </div>
      </aside>

      <section className="flex-1 flex flex-col min-w-0 min-h-0">
        {imageUrl && (
          <div className="h-12 shrink-0 border-b border-border flex items-center justify-between px-5">
            <div className="flex items-center gap-2 min-w-0 text-xs">
              <ImageIcon className="w-3.5 h-3.5 text-muted-foreground shrink-0" />
              <span className="font-mono truncate">{imageName}</span>
              {imageDims && (
                <span className="font-mono text-muted-foreground shrink-0">
                  · {imageDims.w}×{imageDims.h}
                </span>
              )}
            </div>
            <div className="flex items-center gap-3">
              {processedUrl && (
                <ToggleGroup
                  type="single"
                  value={previewMode}
                  onValueChange={(v) => v && setPreviewMode(v as "original" | "result")}
                  size="sm"
                >
                  <ToggleGroupItem value="original">Original</ToggleGroupItem>
                  <ToggleGroupItem value="result">Result</ToggleGroupItem>
                </ToggleGroup>
              )}
              <button onClick={clearImage} className="text-muted-foreground hover:text-foreground transition-colors">
                <X className="w-4 h-4" />
              </button>
            </div>
          </div>
        )}

        <div className="flex-1 flex items-center justify-center p-8 checker-bg min-h-0 min-w-0">
          {displayedUrl ? (
            <img
              src={displayedUrl}
              alt={imageName ?? "preview"}
              className="max-w-full max-h-full object-contain rounded-sm shadow-2xl"
            />
          ) : (
            <div className="text-center">
              <Upload className="w-6 h-6 text-input mx-auto mb-3" />
              <p className="text-sm text-muted-foreground">Drop an image to begin</p>
              <p className="text-xs text-muted-foreground mt-1">or click Browse in the sidebar</p>
            </div>
          )}
        </div>
      </section>
    </main>
  );
}

export default App;
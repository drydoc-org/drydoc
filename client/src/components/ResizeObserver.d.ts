// Copied from https://gist.github.com/ddprrt/4b7c370f72908fe84a2afbe81681d91a

type ResizeObserverBoxOptions = "border-box" | "content-box" | "device-pixel-content-box";

interface ResizeObserverOptions {
    box?: ResizeObserverBoxOptions;
}

interface ResizeObservation {
    readonly lastReportedSizes: ReadonlyArray<ResizeObserverSize>;
    readonly observedBox: ResizeObserverBoxOptions;
    readonly target: Element;
}

declare var ResizeObservation: {
    prototype: ResizeObservation;
    new(target: Element): ResizeObservation;
};

interface ResizeObserver {
    disconnect(): void;
    observe(target: Element, options?: ResizeObserverOptions): void;
    unobserve(target: Element): void;
}

declare var ResizeObserver: {
    prototype: ResizeObserver;
    new(callback: ResizeObserverCallback): ResizeObserver;
};

interface ResizeObserverEntry {
    readonly borderBoxSize: ReadonlyArray<ResizeObserverSize>;
    readonly contentBoxSize: ReadonlyArray<ResizeObserverSize>;
    readonly contentRect: DOMRectReadOnly;
    readonly devicePixelContentBoxSize: ReadonlyArray<ResizeObserverSize>;
    readonly target: Element;
}

declare var ResizeObserverEntry: {
    prototype: ResizeObserverEntry;
    new(): ResizeObserverEntry;
};

interface ResizeObserverSize {
    readonly blockSize: number;
    readonly inlineSize: number;
}

declare var ResizeObserverSize: {
    prototype: ResizeObserverSize;
    new(): ResizeObserverSize;
};

interface ResizeObserverCallback {
    (entries: ResizeObserverEntry[], observer: ResizeObserver): void;
}
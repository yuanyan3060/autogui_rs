class Image {
    save(path: string): void
    show(name: string): void
    height(): number
    width(): number
    match_template(templ: Image, threshold: number = 0.85): Point | null
    static open(path: string): Image
}

class Adb {
    addr: string
    protected bin_path: string
    protected target: string
    constructor(addr: string, target: string, bin_path: string)
    click(x: number, y: number): void
    screenshot(): Image
}

class Point {
    x: number
    y: number
    constructor(x: number, y: number)
}

interface Console {
    debug(...data: any[]): void;
    error(...data: any[]): void;
    info(...data: any[]): void;
    log(...data: any[]): void;
    trace(...data: any[]): void;
    warn(...data: any[]): void;
}

declare var console: Console;

function sleep(ms: number)


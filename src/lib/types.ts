// tip structure

export type AnalysisResult = {
    file_path: string,
    tips: Tip[],
}

export type Tip = {
    property_name: string,
    property_value: string,
    message: string
}

export class ArenaTree {
    nodes_map: Map<string, Node> = new Map<string, Node>();
}

export class Node {
    id: number = 0;
    value: string = "";

    parent: number|null = null;
    alternatives: number[] = [];
    children: number[] = [];
    
    checksum: string = "";
}
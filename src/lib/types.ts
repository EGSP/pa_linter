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


export type Node = {
    id: number,
    value: string,

    parent: number|null,
    alternatives: number[],
    children: number[],
    
    checksum: string
}

export type ArenaTree={
    nodes_map: Map<number, Node>,

}
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
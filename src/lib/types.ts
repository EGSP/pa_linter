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


export type DirectoryImage= {
    name: string,
    files: string[],
}

export type Repository={
    folder_path: string
}

export type RepositoryInfo = {
    folder_path: string,
    mod_name: string
}

export type RepositoryTree={
    entries: RepositoryTreeEntry[],

    repository_info: RepositoryInfo
}

export type RepositoryTreeEntry = {
    id: number,

    path: string,
    parent: number|null,
    children: number[],
}
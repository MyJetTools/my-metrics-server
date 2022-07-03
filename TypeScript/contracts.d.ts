

interface IServicesList {
    services: IService[];
}


interface IService {
    id: String,
    avg: number
}


interface IServiceOverviewList {
    data: IServiceOverview[];
}

interface IServiceOverview {
    data: String;
    min: number;
    max: number;
    avg: number;
    success: number;
    error: number;
    total: number;
}


interface IMetrics {
    metrics: IMetric[];
}

interface IMetric {
    id: number;
    started: number;
    duration: number;
    success: String;
    error: String;
    ip: String
}
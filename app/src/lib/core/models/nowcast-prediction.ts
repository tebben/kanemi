export type NowcastPrediction = {
    datetime: string;
    values: Array<NowcastPredictionValue>;
};

export type NowcastPredictionValue = {
    datetime: string;
    value: number;
};

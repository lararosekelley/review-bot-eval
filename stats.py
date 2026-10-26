def moving_average(values, window):
    """Average of each run of `window` consecutive values."""
    return [sum(values[i:i + window]) / window for i in range(len(values) - window + 1)]


def percentile(values, p):
    """The p-th percentile (0-100) of values, by nearest rank."""
    ordered = sorted(values)
    rank = max(1, round(p / 100 * len(ordered)))
    return ordered[rank - 1]

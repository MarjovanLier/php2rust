<?php

declare(strict_types=1);

final readonly class Order
{
    /** @param list<string> $lines */
    public function __construct(public array $lines)
    {
    }

    public function hasLines(): bool
    {
        return $this->lines !== [];
    }
}

/** @param list<int> $xs */
function has(array $xs): string
{
    if ($xs !== []) {
        return 'some';
    }
    return 'none';
}

/** @param list<string> $xs */
function none(array $xs): bool
{
    return [] === $xs;
}

/** @param array<string, int> $m */
function noKeys(array $m): bool
{
    return ($m === []);
}

function local(int $n): string
{
    $xs = [];
    if ($n > 0) {
        $xs[] = $n;
    }
    return $xs === [] ? 'empty' : 'full';
}

echo has([1]), ' ', has([]), "\n";
echo none([]) ? 'empty' : 'full', ' ', none(['a']) ? 'empty' : 'full', "\n";
echo noKeys([]) ? 'no keys' : 'keys', ' ', noKeys(['a' => 1]) ? 'no keys' : 'keys', "\n";
echo local(0), ' ', local(3), "\n";
echo (new Order(['x']))->hasLines() ? 'lines' : 'no lines', ' ', (new Order([]))->hasLines() ? 'lines' : 'no lines', "\n";

<?php

declare(strict_types=1);

/**
 * @return array<string, int>
 */
function counts(int $b): array
{
    return ['a' => 1, 'b' => $b];
}

/**
 * @param array<string, int>|string $value
 */
function describe(array|string $value): string
{
    if (is_string($value)) {
        return 'text ' . $value;
    }
    return gettype($value);
}

/**
 * @param array<string, int>|string|null $value
 */
function scalarText(array|string|null $value): string
{
    return is_scalar($value) ? (string) $value : gettype($value);
}

/**
 * @return array<string, int>|string
 */
function pick(bool $map): array|string
{
    if ($map) {
        return counts(2);
    }
    return 'none';
}

/**
 * @param array<string, int> $values
 *
 * @return array<string, array<string, int>|bool>
 */
function wrap(array $values, bool $flag): array
{
    return ['values' => $values, 'flag' => $flag];
}

/**
 * @param array<string, int|string>|int $value
 */
function mixed(array|int $value): string
{
    if (is_scalar($value)) {
        return gettype($value);
    }
    return 'map ' . gettype($value);
}

/**
 * @return array<string, int|string>
 */
function labels(): array
{
    return ['n' => 1, 's' => 'one'];
}

/**
 * @param array{a: string, b: string}|array<string, string>|bool $value
 */
function both(array|bool $value): string
{
    return gettype($value);
}

/**
 * @return array{a: string, b: string}
 */
function ab(string $x): array
{
    return ['a' => $x, 'b' => 'B'];
}

/**
 * @return array<string, string>
 */
function cd(string $x): array
{
    return ['c' => $x];
}

echo mixed(7), ' ', mixed(labels()), "\n";
echo both(ab('x')), ' ', both(cd('y')), ' ', both(false), "\n";
echo describe('x'), ' ', describe(counts(1)), "\n";
echo scalarText('y'), ' ', scalarText(counts(2)), ' ', scalarText(null), "\n";
echo describe(pick(true)), ' ', describe(pick(false)), "\n";
echo count(wrap(counts(3), true)), "\n";

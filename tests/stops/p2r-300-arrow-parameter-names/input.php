<?php

declare(strict_types=1);

// PHP rejects each of these parameters when it compiles the file, and Mago reports none.

/**
 * @param list<int> $xs
 * @return list<int>
 */
function query(array $xs): array
{
    return array_map(fn(int $_GET): int => $_GET + 1, $xs);
}

/**
 * @param list<int> $xs
 * @return list<int>
 */
function doubled(array $xs): array
{
    return array_map(static fn(int $GLOBALS): int => $GLOBALS * 2, $xs);
}

/**
 * @param list<int> $xs
 * @return list<int>
 */
function own(array $xs): array
{
    return array_map(fn(int $this): int => $this - 1, $xs);
}

function matched(string $text): string
{
    return preg_replace_callback('#a#', fn(array $_POST): string => $_POST[0], $text) ?? '';
}

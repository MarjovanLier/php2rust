<?php

declare(strict_types=1);

// PHP rejects these parameters when it compiles the file, and Mago reports none of them.
function fromQuery(string $_GET): string
{
    return 'q';
}

function shared(static string $x): string
{
    return $x;
}

function promoted(public string $y): string
{
    return $y;
}

/**
 * @param list<int> $xs
 * @return list<int>
 */
function bump(array $xs): array
{
    return array_map(fn(static int $x): int => $x + 1, $xs);
}

final readonly class Box
{
    public function __construct(public int $_POST, public static int $n, public int $this) {}
}

// A catch variable may not hide a function either.
function check(): string
{
    try {
        throw new RuntimeException('x');
    } catch (RuntimeException $check) {
        return 'caught';
    }
}

// Nor may it take a superglobal's name, which PHP overwrites (P2R-214).
function query(): string
{
    try {
        throw new RuntimeException('m');
    } catch (RuntimeException $_GET) {
        return $_GET->getMessage();
    }
}

// PHP rejects $this as a catch variable.
function self_catch(): string
{
    try {
        throw new RuntimeException('m');
    } catch (RuntimeException $this) {
        return 'caught';
    }
}

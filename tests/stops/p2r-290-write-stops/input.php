<?php

declare(strict_types=1);

function maybe(string $s): ?string
{
    if ($s === '') {
        return null;
    }
    return $s;
}

function inBody(?string $v): string
{
    if ($v !== null) {
        $v = $v . '!';
        return $v;
    }
    return 'none';
}

function notLast(?string $v, bool $flag): string
{
    if ($v === null) {
        if ($flag) {
            return 'a';
        } else {
            return 'b';
        }
    }
    $v = $v . '!';
    return $v;
}

function mayBeNull(?string $v): string
{
    if ($v === null) {
        return 'none';
    }
    $v = maybe($v);
    return $v ?? 'none';
}

function toNull(?string $v): string
{
    if ($v === null) {
        return 'none';
    }
    $v = null;
    return 'gone';
}

function counted(?int $n): int
{
    if ($n === null) {
        return 0;
    }
    $n = null;
    return 1;
}

function tested(int|string $u): string
{
    if (!is_string($u)) {
        return 'int';
    }
    $u = $u . '!';
    return $u;
}

/**
 * @param ?array{a: int} $p
 */
function shaped(?array $p): int
{
    if ($p === null) {
        return 0;
    }
    $p = ['a' => 2];
    return $p['a'];
}

function unread(?string $v): string
{
    if ($v === null) {
        return '';
    }
    $v = 'x';
    return $v;
}

/**
 * @param list<string> $xs
 */
function looped(?string $v, array $xs): string
{
    if ($v === null) {
        return 'none';
    }
    $v .= '!';
    foreach ($xs as $v) {
        echo $v;
    }
    return 'done';
}

echo inBody('a'), notLast('a', true), mayBeNull('a'), toNull('a'), counted(1), tested('a'),
    shaped(['a' => 1]), unread('a'), looped('a', ['b']), "\n";

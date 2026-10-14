<?php

declare(strict_types=1);

final class Slug
{
    public function make(string $text, ?string $suffix): string
    {
        $text = strtolower(trim($text));
        $text = str_replace(' ', '-', $text);
        if ($suffix !== null) {
            $text .= '-' . $suffix;
        }
        return $text;
    }

    public function orNone(?string $name): string
    {
        if ($name === null) {
            $name = 'none';
        }
        return $name ?? '';
    }

    public static function twice(string $a, string $b): string
    {
        $a .= $a;
        $b = $a . $b;
        return $b;
    }
}

/**
 * @param list<int> $xs
 * @return list<int>
 */
function withZero(array $xs): array
{
    $xs[] = 0;
    return $xs;
}

function shout(string $s, bool $loud): string
{
    if ($loud) {
        $s .= '!';
    }
    return $s;
}

/** @param list<string> $xs */
function last(array $xs, string $x): string
{
    $x .= '?';
    echo $x, ' ';
    foreach ($xs as $x) {
        echo $x, ' ';
    }
    return 'end';
}

$original = 'Keep';
echo shout($original, true), ' ', $original, "\n";
echo (new Slug())->make('  Hello World ', 'v2'), ' ', (new Slug())->make('A B', null), "\n";
echo (new Slug())->orNone(null), ' ', (new Slug())->orNone('x'), "\n";
echo Slug::twice('ab', 'c'), "\n";
echo last(['p', 'q'], 'z'), ' ', last([], 'y'), "\n";
$list = [1, 2];
echo count(withZero($list)), ' ', count($list), "\n";

<?php

declare(strict_types=1);

function maybe(int $n): ?string
{
    if ($n > 0) {
        return "x";
    }
    return null;
}

function maybeInt(int $n): ?int
{
    if ($n > 0) {
        return $n;
    }
    return null;
}

$name = maybe(1);
if ($name === null) {
    throw new RuntimeException("no name");
}
$name = strtoupper($name);
$name .= "!";
echo $name, "\n";

$count = maybeInt(2);
if (null === $count) {
    throw new RuntimeException("no count");
}
$count += 1;
if ($count > 2) {
    $count++;
}
echo $count, "\n";

<?php

declare(strict_types=1);

final readonly class Path
{
    public function __construct(public string $text) {}

    public function absolute(): bool
    {
        return str_starts_with($this->text, "/");
    }
}

function kind(string $name, string $suffix): string
{
    $owned = $name . ".bak";
    if (str_ends_with($owned, ".bak") && str_ends_with($name, $suffix)) {
        return "backup of " . $suffix;
    }
    if (str_contains($name, "") && str_contains($owned, $name)) {
        return "plain";
    }
    return "other";
}

echo kind("notes.txt", ".txt"), "\n";
echo kind("notes.md", ".txt"), "\n";
if ((new Path("/tmp"))->absolute() && !(new Path("tmp"))->absolute()) {
    echo "absolute\n";
}
if (str_starts_with("abc", "") && !str_contains("abc", "B")) {
    echo "bytes\n";
}

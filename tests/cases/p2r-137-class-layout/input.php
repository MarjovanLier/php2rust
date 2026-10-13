<?php

declare(strict_types=1);

readonly class Short
{
    public function __construct(public int $aaaaaaa, public int $bbbbbbbbb)
    {
    }
}

readonly class Longer
{
    public function __construct(public int $aaaaaaaa, public int $bbbbbbbbb)
    {
    }
}

readonly class Text
{
    public function __construct(public string $s)
    {
    }
}

readonly class Bare
{
}

function size(Bare $bare): int
{
    return 1;
}

$short = new Short(1, 2);
$longer = new Longer(3, 4);
$text = new Text('t');
echo $short->aaaaaaa + $longer->bbbbbbbbb, $text->s, size(new Bare()), "\n";

<?php

declare(strict_types=1);

final class Node
{
    public int $n = 0;
    public ?int $m = null;
    /** @var list<int> */
    public array $xs = [];
    /** @var list<float> */
    public array $fs = [];
}

function make(): Node
{
    return new Node();
}

function throughCall(): void
{
    make()->n = 1;
}

function coalesce(Node $a): void
{
    $a->m ??= 1;
}

function element(Node $a): void
{
    $a->xs[0] = 1;
}

function asValue(Node $a): int
{
    return $a->n = 2;
}

function intIntoFloats(Node $a): void
{
    $a->fs[] = 1;
}

function concatInt(Node $a): void
{
    $a->n .= "x";
}

function appendInt(Node $a): void
{
    $a->n[] = 1;
}

function indirect(Node $a): void
{
    $name = 'a';
    ${$name}->n = 1;
}

function floatList(Node $a): void
{
    $a->fs = [1, 2];
}

function maybe(Node $a, ?int $v): void
{
    $a->n = $v;
}

function neverRead(): void
{
    $a = new Node();
    $a->n = 1;
}

function neverReadInLoop(int $k): void
{
    $a = new Node();
    echo $a->n, "\n";
    for ($i = 0; $i < $k; $i++) {
        $a->n = $i;
    }
}

/**
 * @param list<Node> $nodes
 */
function replace(array $nodes): void
{
    foreach ($nodes as $node) {
        echo $node->n, "\n";
        $node = new Node();
        echo $node->n, "\n";
    }
}

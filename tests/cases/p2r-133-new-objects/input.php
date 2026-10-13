<?php

declare(strict_types=1);

readonly class Tag
{
    public function __construct(public string $text)
    {
    }

    public function shout(): string
    {
        return $this->text . '!';
    }
}

readonly class Post
{
    /** @param list<string> $words */
    public function __construct(public Tag $tag, public array $words, public ?string $note, public ?int $likes)
    {
    }
}

readonly class Flag
{
}

function first(Flag $flag): string
{
    return 'flag';
}

$tag = new Tag('news');
$post = new Post($tag, ['a', 'b'], null, 3);
echo $post->tag->text, ' ', count($post->words), "\n";
echo new Tag('x')->shout(), "\n";
echo first(new Flag), "\n";

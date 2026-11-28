<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

// An Eloquent model whose columns come from the migration, written by hand for the harness.
final class Post extends Model
{
    protected $fillable = ['title', 'views'];

    public $timestamps = false;
}

function created(string $title, int $views): int
{
    $post = Post::create(['title' => $title, 'views' => $views]);
    return $post->id;
}

function titleOf(int $id): string
{
    $post = Post::query()->find($id);
    return $post === null ? 'none' : $post->title;
}

function viewsOf(int $id): ?int
{
    $post = Post::query()->find($id);
    return $post === null ? null : $post->views;
}

/**
 * @return list<string>
 */
function titles(): array
{
    $found = [];
    foreach (Post::all() as $post) {
        $found[] = $post->title;
    }
    return $found;
}

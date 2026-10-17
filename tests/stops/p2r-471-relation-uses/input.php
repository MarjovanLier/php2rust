<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;

final class Author extends Model
{
    protected $fillable = ['name'];

    public $timestamps = false;

    public function posts(): HasMany
    {
        return $this->hasMany(Post::class);
    }
}

final class Post extends Model
{
    protected $fillable = ['title', 'author_id'];

    public $timestamps = false;

    public function author(): BelongsTo
    {
        return $this->belongsTo(Author::class);
    }
}

// A call of a relation method, also as a statement, and a read of a relation as a
// property stop (P2R-471).
function relation(Author $author): void
{
    $author->posts();
}

function owned(Post $post): bool
{
    return $post->author === null;
}

function follow(Post $post): void
{
    $post->author();
}

<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasManyThrough;
use Illuminate\Database\Eloquent\Relations\HasOne;
use Illuminate\Database\Eloquent\Relations\HasOneThrough;
use Illuminate\Database\Eloquent\Relations\MorphMany;
use Illuminate\Database\Eloquent\Relations\MorphOne;
use Illuminate\Database\Eloquent\Relations\MorphTo;

// Each helper of P2R-470 with the return type it builds, its arguments positional string
// literals or X::class; the methods leave the output, as casts() does, and the models keep
// their structs.
final class Author extends Model
{
    protected $fillable = ['name'];

    public $timestamps = false;

    public function posts(): HasMany
    {
        return $this->hasMany(Post::class);
    }

    public function latestPost(): HasOne
    {
        return $this->hasOne(Post::class, 'author_id', 'id');
    }

    public function editors(): BelongsToMany
    {
        return $this->belongsToMany(Author::class, 'author_editor');
    }

    public function postImages(): HasManyThrough
    {
        return $this->hasManyThrough(Image::class, Post::class);
    }

    public function firstImage(): HasOneThrough
    {
        return $this->hasOneThrough(Image::class, Post::class);
    }

    public function images(): MorphMany
    {
        return $this->morphMany(Image::class, 'imageable');
    }

    public function avatar(): MorphOne
    {
        return $this->morphOne(Image::class, "imageable");
    }
}

// A fully qualified return type, and a relation method beside casts().
final class Post extends Model
{
    protected $fillable = ['title', 'author_id'];

    public $timestamps = false;

    public function author(): \Illuminate\Database\Eloquent\Relations\BelongsTo
    {
        return $this->belongsTo(Author::class);
    }

    protected function casts(): array
    {
        return ['author_id' => 'integer'];
    }
}

// morphTo takes no argument.
final class Image extends Model
{
    protected $fillable = ['url', 'imageable_type', 'imageable_id'];

    public $timestamps = false;

    public function imageable(): MorphTo
    {
        return $this->morphTo();
    }
}

function title(int $id): string
{
    $post = Post::find($id);
    return $post === null ? '-' : $post->title;
}

$author = Author::create(['name' => 'Ann']);
$post = Post::create(['title' => 'First', 'author_id' => $author->id]);
$image = Image::create(['url' => 'a.png', 'imageable_type' => 'post', 'imageable_id' => $post->id]);
echo $author->id, ' ', $author->name, ' ', $post->author_id, ' ', $image->url, "\n";
echo title(1), ' ', title(2), "\n";

<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

final class Post extends Model
{
    protected $fillable = ['title', 'views'];

    public $timestamps = false;
}

// findOrFail on a missing row ends the program with Laravel's uncaught exception, exit 255.
$post = Post::create(['title' => 'only', 'views' => 1]);
echo Post::query()->findOrFail($post->id)->title, "\n";
echo Post::query()->findOrFail(9)->title, "\n";

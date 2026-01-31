defmodule Kreuzberg.Chunk do
  @moduledoc """
  Structure representing a text chunk with embedding for semantic search.

  Matches the Rust `Chunk` struct.

  ## Fields

    * `:content` - The text content of this chunk
    * `:embedding` - Vector embedding (list of floats) for semantic search
    * `:metadata` - ChunkMetadata struct with position and token info
  """

  @type t :: %__MODULE__{
          content: String.t(),
          embedding: list(float()) | nil,
          metadata: Kreuzberg.ChunkMetadata.t()
        }

  defstruct content: "", embedding: nil, metadata: %Kreuzberg.ChunkMetadata{}

  @doc """
  Creates a new Chunk struct.

  ## Parameters

    * `content` - The text content of the chunk
    * `opts` - Optional keyword list with `:embedding` and `:metadata`
  """
  @spec new(String.t(), keyword()) :: t()
  def new(content, opts \\ []) when is_binary(content) do
    %__MODULE__{
      content: content,
      embedding: Keyword.get(opts, :embedding),
      metadata: Keyword.get(opts, :metadata, %Kreuzberg.ChunkMetadata{})
    }
  end

  @doc """
  Creates a Chunk struct from a map.

  ## Examples

      iex> Kreuzberg.Chunk.from_map(%{"content" => "chunk text", "embedding" => [0.1, 0.2]})
      %Kreuzberg.Chunk{content: "chunk text", embedding: [0.1, 0.2]}
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    metadata =
      case data["metadata"] do
        nil -> %Kreuzberg.ChunkMetadata{}
        %Kreuzberg.ChunkMetadata{} = m -> m
        map when is_map(map) -> Kreuzberg.ChunkMetadata.from_map(map)
      end

    %__MODULE__{
      content: data["content"] || "",
      embedding: data["embedding"],
      metadata: metadata
    }
  end

  @doc """
  Converts a Chunk struct to a map.
  """
  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = chunk) do
    %{
      "content" => chunk.content,
      "embedding" => chunk.embedding,
      "metadata" => Kreuzberg.ChunkMetadata.to_map(chunk.metadata)
    }
  end
end

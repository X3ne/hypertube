/* eslint-disable */
/* tslint:disable */
/*
 * ---------------------------------------------------------------
 * ## THIS FILE WAS GENERATED VIA SWAGGER-TYPESCRIPT-API        ##
 * ##                                                           ##
 * ## AUTHOR: acacode                                           ##
 * ## SOURCE: https://github.com/acacode/swagger-typescript-api ##
 * ---------------------------------------------------------------
 */

/** AddTorrentWithMagnet */
export interface AddTorrentWithMagnet {
  magnet: string;
}

export interface AlternativeTitle {
  iso_3166_1: string;
  title: string;
  type: string;
}

export interface Cast {
  /**
   * @format uint32
   * @min 0
   */
  cast_id: number;
  character: string;
  /**
   * @format uint8
   * @min 0
   */
  gender?: number | null;
  /**
   * @format uint32
   * @min 0
   */
  id: number;
  name: string;
  /**
   * @format uint8
   * @min 0
   */
  order: number;
  profile_path?: string | null;
}

export interface Credits {
  cast: Cast[];
  crew: Crew[];
}

export interface Crew {
  department: string;
  /**
   * @format uint8
   * @min 0
   */
  gender?: number | null;
  /**
   * @format uint32
   * @min 0
   */
  id: number;
  job: string;
  name: string;
  profile_path?: string | null;
}

export interface Episode {
  air_date: string;
  crew: Crew[];
  /** @format int64 */
  episode_number: number;
  episode_type: string;
  /** @default [] */
  guest_stars?: TVCast[];
  /** @format int64 */
  id: number;
  name: string;
  overview: string;
  production_code: string;
  /** @format int64 */
  runtime?: number | null;
  /** @format int64 */
  season_number: number;
  /** @format int64 */
  show_id: number;
  still_path?: string | null;
  /** @format double */
  vote_average: number;
  /** @format int64 */
  vote_count: number;
}

export interface FindMovie {
  adult: boolean;
  backdrop_path?: string | null;
  /** @default [] */
  genre_ids?: number[];
  /**
   * @format uint32
   * @min 0
   */
  id: number;
  original_language: string;
  original_title: string;
  overview?: string | null;
  /** @format double */
  popularity?: number | null;
  poster_path?: string | null;
  release_date?: string | null;
  title: string;
}

export interface FindTV {
  adult: boolean;
  backdrop_path?: string | null;
  first_air_date?: string | null;
  /** @default [] */
  genre_ids?: number[];
  /**
   * @format uint32
   * @min 0
   */
  id: number;
  name: string;
  original_language: string;
  original_name: string;
  overview?: string | null;
  /** @format double */
  popularity?: number | null;
  poster_path?: string | null;
  /** @format float */
  vote_average?: number | null;
  /**
   * @format uint32
   * @min 0
   */
  vote_count?: number | null;
}

export interface Genre {
  /**
   * @format uint16
   * @min 0
   */
  id: number;
  name: string;
}

export interface LastEpisode {
  air_date: string;
  /**
   * @format uint32
   * @min 0
   */
  episode_number: number;
  /**
   * @format uint32
   * @min 0
   */
  id: number;
  name: string;
  overview: string;
  production_code?: string | null;
  /**
   * @format uint32
   * @min 0
   */
  season_number: number;
  still_path?: string | null;
  /** @format double */
  vote_average: number;
  /**
   * @format uint64
   * @min 0
   */
  vote_count: number;
}

/** Movie */
export interface Movie {
  adult: boolean;
  backdrop_path?: string | null;
  /**
   * @format uint64
   * @min 0
   */
  budget?: number | null;
  credits?: Credits | null;
  /** @default [] */
  genres?: Genre[];
  homepage?: string | null;
  /**
   * @format uint32
   * @min 0
   */
  id: number;
  /**
   * @format uint32
   * @min 0
   */
  imdb_id?: number | null;
  original_language: string;
  original_title: string;
  overview?: string | null;
  /** @format double */
  popularity: number;
  poster_path?: string | null;
  release_date: string;
  /**
   * @format uint32
   * @min 0
   */
  runtime?: number | null;
  /** @default "Movie" */
  show_type?: ShowType;
  tagline?: string | null;
  title: string;
  videos?: ResultsForVideo | null;
}

export interface Network {
  /**
   * @format uint32
   * @min 0
   */
  id: number;
  logo_path?: string | null;
  name: string;
  origin_country?: string | null;
}

export interface ProductionCompany {
  /**
   * @format uint32
   * @min 0
   */
  id: number;
  logo_path?: string | null;
  name: string;
  origin_country?: string | null;
}

export interface ResultsForAlternativeTitle {
  results: AlternativeTitle[];
}

export interface ResultsForVideo {
  results: Video[];
}

/** SearchResults */
export interface SearchResults {
  movies: FindMovie[];
  tv: FindTV[];
}

export interface Season {
  air_date?: string | null;
  /**
   * @format uint32
   * @min 0
   */
  episode_count: number;
  /**
   * @format uint32
   * @min 0
   */
  id: number;
  name: string;
  overview: string;
  poster_path?: string | null;
  /**
   * @format uint32
   * @min 0
   */
  season_number: number;
}

/** ServerHealth */
export interface ServerHealth {
  status: string;
}

export enum ShowType {
  Movie = "Movie",
  TV = "TV",
}

export enum Source {
  Value1337X = "1337x",
  Yts = "Yts",
  Eztv = "Eztv",
  TorrentGalaxy = "TorrentGalaxy",
  Torlock = "Torlock",
  PirateBay = "PirateBay",
  Nyaasi = "Nyaasi",
  Rarbg = "Rarbg",
  Ettv = "Ettv",
  Zooqle = "Zooqle",
  KickAss = "KickAss",
  Bitsearch = "Bitsearch",
  Glodls = "Glodls",
  MagnetDL = "MagnetDL",
  LimeTorrent = "LimeTorrent",
  TorrentFunk = "TorrentFunk",
  TorrentProject = "TorrentProject",
  Prowlarr = "Prowlarr",
  Unknown = "Unknown",
}

/** TV */
export interface TV {
  alternative_titles?: ResultsForAlternativeTitle | null;
  backdrop_path?: string | null;
  /** @default [] */
  created_by?: TVCreator[];
  credits?: TVCredits | null;
  /** @default [] */
  episode_run_time?: number[];
  first_air_date: string;
  /** @default [] */
  genres?: Genre[];
  homepage?: string | null;
  /**
   * @format uint32
   * @min 0
   */
  id: number;
  /** @default false */
  in_production?: boolean;
  /** @default [] */
  languages?: string[];
  last_air_date?: string | null;
  last_episode_to_air?: LastEpisode | null;
  name: string;
  /** @default [] */
  networks?: Network[];
  /**
   * @format uint32
   * @min 0
   * @default 0
   */
  number_of_episodes?: number;
  /**
   * @format uint32
   * @min 0
   * @default 0
   */
  number_of_seasons?: number;
  /** @default [] */
  origin_country?: string[];
  original_language: string;
  original_name: string;
  overview: string;
  /** @format double */
  popularity: number;
  poster_path?: string | null;
  /** @default [] */
  production_companies?: ProductionCompany[];
  /** @default [] */
  seasons?: Season[];
  /** @default "Movie" */
  show_type?: ShowType;
  status?: string | null;
  type?: string | null;
  videos?: ResultsForVideo | null;
  /** @format double */
  vote_average: number;
  /**
   * @format uint64
   * @min 0
   */
  vote_count: number;
}

export interface TVCast {
  character: string;
  /**
   * @format uint8
   * @min 0
   */
  gender?: number | null;
  /**
   * @format uint32
   * @min 0
   */
  id: number;
  name: string;
  /**
   * @format uint32
   * @min 0
   */
  order: number;
  profile_path?: string | null;
}

export interface TVCreator {
  /**
   * @format uint8
   * @min 0
   */
  gender?: number | null;
  /**
   * @format uint32
   * @min 0
   */
  id: number;
  name: string;
  profile_path?: string | null;
}

export interface TVCredits {
  cast: TVCast[];
  crew: Crew[];
}

/** Torrent */
export interface Torrent {
  /** @format int64 */
  downloads: number;
  /** @format int64 */
  leechers: number;
  magnet: string;
  name: string;
  /** @format int64 */
  seeders: number;
  size: string;
  source: Source;
  torrent?: string | null;
}

/** TvSeason */
export interface TvSeason {
  air_date: string;
  /** @default [] */
  episodes?: Episode[];
  /** @format int64 */
  id: number;
  name: string;
  overview: string;
  poster_path: string;
  /** @format int64 */
  season_number: number;
  /** @format double */
  vote_average: number;
}

export interface Video {
  id: string;
  iso_3166_1: string;
  iso_639_1: string;
  key: string;
  name: string;
  official: boolean;
  published_at: string;
  site: string;
  /**
   * @format uint32
   * @min 0
   */
  size: number;
  type: string;
}

export type QueryParamsType = Record<string | number, any>;
export type ResponseFormat = keyof Omit<Body, "body" | "bodyUsed">;

export interface FullRequestParams extends Omit<RequestInit, "body"> {
  /** set parameter to `true` for call `securityWorker` for this request */
  secure?: boolean;
  /** request path */
  path: string;
  /** content type of request body */
  type?: ContentType;
  /** query params */
  query?: QueryParamsType;
  /** format of response (i.e. response.json() -> format: "json") */
  format?: ResponseFormat;
  /** request body */
  body?: unknown;
  /** base url */
  baseUrl?: string;
  /** request cancellation token */
  cancelToken?: CancelToken;
}

export type RequestParams = Omit<FullRequestParams, "body" | "method" | "query" | "path">;

export interface ApiConfig<SecurityDataType = unknown> {
  baseUrl?: string;
  baseApiParams?: Omit<RequestParams, "baseUrl" | "cancelToken" | "signal">;
  securityWorker?: (securityData: SecurityDataType | null) => Promise<RequestParams | void> | RequestParams | void;
  customFetch?: typeof fetch;
}

export interface HttpResponse<D extends unknown, E extends unknown = unknown> extends Response {
  data: D;
  error: E;
}

type CancelToken = Symbol | string | number;

export enum ContentType {
  Json = "application/json",
  FormData = "multipart/form-data",
  UrlEncoded = "application/x-www-form-urlencoded",
  Text = "text/plain",
}

export class HttpClient<SecurityDataType = unknown> {
  public baseUrl: string = "";
  private securityData: SecurityDataType | null = null;
  private securityWorker?: ApiConfig<SecurityDataType>["securityWorker"];
  private abortControllers = new Map<CancelToken, AbortController>();
  private customFetch = (...fetchParams: Parameters<typeof fetch>) => fetch(...fetchParams);

  private baseApiParams: RequestParams = {
    credentials: "same-origin",
    headers: {},
    redirect: "follow",
    referrerPolicy: "no-referrer",
  };

  constructor(apiConfig: ApiConfig<SecurityDataType> = {}) {
    Object.assign(this, apiConfig);
  }

  public setSecurityData = (data: SecurityDataType | null) => {
    this.securityData = data;
  };

  protected encodeQueryParam(key: string, value: any) {
    const encodedKey = encodeURIComponent(key);
    return `${encodedKey}=${encodeURIComponent(typeof value === "number" ? value : `${value}`)}`;
  }

  protected addQueryParam(query: QueryParamsType, key: string) {
    return this.encodeQueryParam(key, query[key]);
  }

  protected addArrayQueryParam(query: QueryParamsType, key: string) {
    const value = query[key];
    return value.map((v: any) => this.encodeQueryParam(key, v)).join("&");
  }

  protected toQueryString(rawQuery?: QueryParamsType): string {
    const query = rawQuery || {};
    const keys = Object.keys(query).filter((key) => "undefined" !== typeof query[key]);
    return keys
      .map((key) => (Array.isArray(query[key]) ? this.addArrayQueryParam(query, key) : this.addQueryParam(query, key)))
      .join("&");
  }

  protected addQueryParams(rawQuery?: QueryParamsType): string {
    const queryString = this.toQueryString(rawQuery);
    return queryString ? `?${queryString}` : "";
  }

  private contentFormatters: Record<ContentType, (input: any) => any> = {
    [ContentType.Json]: (input: any) =>
      input !== null && (typeof input === "object" || typeof input === "string") ? JSON.stringify(input) : input,
    [ContentType.Text]: (input: any) => (input !== null && typeof input !== "string" ? JSON.stringify(input) : input),
    [ContentType.FormData]: (input: any) =>
      Object.keys(input || {}).reduce((formData, key) => {
        const property = input[key];
        formData.append(
          key,
          property instanceof Blob
            ? property
            : typeof property === "object" && property !== null
              ? JSON.stringify(property)
              : `${property}`,
        );
        return formData;
      }, new FormData()),
    [ContentType.UrlEncoded]: (input: any) => this.toQueryString(input),
  };

  protected mergeRequestParams(params1: RequestParams, params2?: RequestParams): RequestParams {
    return {
      ...this.baseApiParams,
      ...params1,
      ...(params2 || {}),
      headers: {
        ...(this.baseApiParams.headers || {}),
        ...(params1.headers || {}),
        ...((params2 && params2.headers) || {}),
      },
    };
  }

  protected createAbortSignal = (cancelToken: CancelToken): AbortSignal | undefined => {
    if (this.abortControllers.has(cancelToken)) {
      const abortController = this.abortControllers.get(cancelToken);
      if (abortController) {
        return abortController.signal;
      }
      return void 0;
    }

    const abortController = new AbortController();
    this.abortControllers.set(cancelToken, abortController);
    return abortController.signal;
  };

  public abortRequest = (cancelToken: CancelToken) => {
    const abortController = this.abortControllers.get(cancelToken);

    if (abortController) {
      abortController.abort();
      this.abortControllers.delete(cancelToken);
    }
  };

  public request = async <T = any, E = any>({
    body,
    secure,
    path,
    type,
    query,
    format,
    baseUrl,
    cancelToken,
    ...params
  }: FullRequestParams): Promise<HttpResponse<T, E>> => {
    const secureParams =
      ((typeof secure === "boolean" ? secure : this.baseApiParams.secure) &&
        this.securityWorker &&
        (await this.securityWorker(this.securityData))) ||
      {};
    const requestParams = this.mergeRequestParams(params, secureParams);
    const queryString = query && this.toQueryString(query);
    const payloadFormatter = this.contentFormatters[type || ContentType.Json];
    const responseFormat = format || requestParams.format;

    return this.customFetch(`${baseUrl || this.baseUrl || ""}${path}${queryString ? `?${queryString}` : ""}`, {
      ...requestParams,
      headers: {
        ...(requestParams.headers || {}),
        ...(type && type !== ContentType.FormData ? { "Content-Type": type } : {}),
      },
      signal: (cancelToken ? this.createAbortSignal(cancelToken) : requestParams.signal) || null,
      body: typeof body === "undefined" || body === null ? null : payloadFormatter(body),
    }).then(async (response) => {
      const r = response.clone() as HttpResponse<T, E>;
      r.data = null as unknown as T;
      r.error = null as unknown as E;

      const data = !responseFormat
        ? r
        : await response[responseFormat]()
            .then((data) => {
              if (r.ok) {
                r.data = data;
              } else {
                r.error = data;
              }
              return r;
            })
            .catch((e) => {
              r.error = e;
              return r;
            });

      if (cancelToken) {
        this.abortControllers.delete(cancelToken);
      }

      if (!response.ok) throw data;
      return data;
    });
  };
}

/**
 * @title Hypertube API
 * @version 0.1.0
 */
export class Api<SecurityDataType extends unknown> extends HttpClient<SecurityDataType> {
  api = {
    /**
     * No description
     *
     * @tags health
     * @name Health
     * @request GET:/api
     */
    health: (params: RequestParams = {}) =>
      this.request<ServerHealth, void>({
        path: `/api`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @tags torrents
     * @name AddTorrentWithMagnet
     * @summary Add a new torrents with a magnet link
     * @request POST:/api/torrents/magnet
     */
    addTorrentWithMagnet: (data: AddTorrentWithMagnet, params: RequestParams = {}) =>
      this.request<void, void>({
        path: `/api/torrents/magnet`,
        method: "POST",
        body: data,
        type: ContentType.Json,
        ...params,
      }),

    /**
     * No description
     *
     * @tags shows
     * @name SearchShows
     * @summary Search for shows
     * @request GET:/api/shows/search
     */
    searchShows: (
      query: {
        query: string;
      },
      params: RequestParams = {},
    ) =>
      this.request<SearchResults, void>({
        path: `/api/shows/search`,
        method: "GET",
        query: query,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @tags shows
     * @name GetTvTrending
     * @summary Get trending TV shows this week
     * @request GET:/api/shows/tv/trending
     */
    getTvTrending: (params: RequestParams = {}) =>
      this.request<TV[], void>({
        path: `/api/shows/tv/trending`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @tags shows
     * @name GetTv
     * @summary Get TV show metadata
     * @request GET:/api/shows/tv/{id}
     */
    getTv: (id: number, params: RequestParams = {}) =>
      this.request<TV, void>({
        path: `/api/shows/tv/${id}`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @tags shows
     * @name GetTvSeason
     * @summary Get TV show season metadata
     * @request GET:/api/shows/tv/{id}/season/{season_number}
     */
    getTvSeason: (id: number, seasonNumber: number, params: RequestParams = {}) =>
      this.request<TvSeason, void>({
        path: `/api/shows/tv/${id}/season/${seasonNumber}`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @tags shows
     * @name GetTvTorrents
     * @summary Get torrents for TV show
     * @request GET:/api/shows/tv/{id}/torrent
     */
    getTvTorrents: (
      id: number,
      query?: {
        params?: string | null;
      },
      params: RequestParams = {},
    ) =>
      this.request<Torrent[], void>({
        path: `/api/shows/tv/${id}/torrent`,
        method: "GET",
        query: query,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @tags shows
     * @name GetMovieTrending
     * @summary Get trending movies this week
     * @request GET:/api/shows/movies/trending
     */
    getMovieTrending: (params: RequestParams = {}) =>
      this.request<Movie[], void>({
        path: `/api/shows/movies/trending`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @tags shows
     * @name GetMovie
     * @summary Get movie metadata
     * @request GET:/api/shows/movies/{id}
     */
    getMovie: (id: number, params: RequestParams = {}) =>
      this.request<Movie, void>({
        path: `/api/shows/movies/${id}`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @tags transcode
     * @name GetManifest
     * @summary Get the mpd manifest
     * @request GET:/api/transcode/start.mpd
     */
    getManifest: (
      query: {
        session_id: string;
      },
      params: RequestParams = {},
    ) =>
      this.request<void, void>({
        path: `/api/transcode/start.mpd`,
        method: "GET",
        query: query,
        ...params,
      }),

    /**
     * No description
     *
     * @tags transcode
     * @name GetInitSegment
     * @summary Get the initialization segment for MPEG-DASH
     * @request GET:/api/transcode/session/{session_id}/{representation_id}/header
     */
    getInitSegment: (sessionId: string, representationId: number, params: RequestParams = {}) =>
      this.request<void, void>({
        path: `/api/transcode/session/${sessionId}/${representationId}/header`,
        method: "GET",
        ...params,
      }),

    /**
     * No description
     *
     * @tags transcode
     * @name GetSegment
     * @summary Get a media segment
     * @request GET:/api/transcode/session/{session_id}/{representation_id}/{segment_number}.m4s
     */
    getSegment: (sessionId: string, representationId: number, segmentNumber: number, params: RequestParams = {}) =>
      this.request<void, void>({
        path: `/api/transcode/session/${sessionId}/${representationId}/${segmentNumber}.m4s`,
        method: "GET",
        ...params,
      }),
  };
}
